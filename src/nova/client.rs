use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json;
use thiserror::Error;
use tokio::time::sleep;

use super::{dto::NovaRequest, dto::NovaResponse, dto::NovaErrorResponse, helpers};

#[derive(Clone)]
pub struct NovaClient {
    http_client: Client,
    base_url: String,
    api_key: String,
}

#[derive(Debug, Error)]
pub enum NovaClientError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("failed to build request headers: {0}")]
    Header(#[from] reqwest::header::InvalidHeaderValue),
    #[error("nova gateway error ({status}): {message}")]
    Gateway { status: u16, message: String },
}

impl NovaClient {
    pub fn new(api_key: String, base_url: String, timeout_secs: u64) -> Result<Self, NovaClientError> {
        let sanitized_base = base_url.trim_end_matches('/').to_string();
        let http_client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;

        Ok(Self {
            http_client,
            base_url: sanitized_base,
            api_key,
        })
    }

    pub async fn send_prompt(&self, request: NovaRequest) -> Result<NovaResponse, NovaClientError> {
        let url = format!("{}/ai", self.base_url);
        let headers = helpers::build_headers(&self.api_key)?;
        let mut attempts = 0u8;

        loop {
            attempts = attempts.saturating_add(1);
            let response = self
                .http_client
                .post(&url)
                .headers(headers.clone())
                .json(&request)
                .send()
                .await?;

            let status = response.status();

            if status == StatusCode::TOO_MANY_REQUESTS && attempts < 3 {
                sleep(Duration::from_millis(500 * attempts as u64)).await;
                continue;
            }

            if status.is_success() {
                return response.json::<NovaResponse>().await.map_err(NovaClientError::from);
            }

            let response_text = response.text().await.unwrap_or_else(|_| String::new());
            let message = match serde_json::from_str::<NovaErrorResponse>(&response_text) {
                Ok(payload) => payload
                    .message
                    .unwrap_or_else(|| {
                        if response_text.is_empty() {
                            format!("request failed with status {}", status.as_u16())
                        } else {
                            format!("request failed with status {}: {}", status.as_u16(), response_text)
                        }
                    }),
                Err(_) => {
                    if response_text.is_empty() {
                        format!("request failed with status {}", status.as_u16())
                    } else {
                        format!("request failed with status {}: {}", status.as_u16(), response_text)
                    }
                }
            };

            return Err(NovaClientError::Gateway {
                status: status.as_u16(),
                message,
            });
        }
    }

    pub async fn clear_history(&self, ref_id: Option<String>) -> Result<(), NovaClientError> {
        let url = format!("{}/ai", self.base_url);
        let headers = helpers::build_headers(&self.api_key)?;
        let mut request = self.http_client.delete(&url).headers(headers);

        if let Some(identifier) = ref_id {
            request = request.query(&[("ref_id", identifier)]);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let response_text = response.text().await.unwrap_or_else(|_| String::new());
            let message = match serde_json::from_str::<NovaErrorResponse>(&response_text) {
                Ok(payload) => payload
                    .message
                    .unwrap_or_else(|| {
                        if response_text.is_empty() {
                            format!("failed to clear history: status {}", status.as_u16())
                        } else {
                            format!("failed to clear history: status {}: {}", status.as_u16(), response_text)
                        }
                    }),
                Err(_) => {
                    if response_text.is_empty() {
                        format!("failed to clear history: status {}", status.as_u16())
                    } else {
                        format!("failed to clear history: status {}: {}", status.as_u16(), response_text)
                    }
                }
            };

            Err(NovaClientError::Gateway {
                status: status.as_u16(),
                message,
            })
        }
    }
}
