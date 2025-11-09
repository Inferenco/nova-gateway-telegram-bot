use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

use crate::config::dto::ReasoningSettings;

use super::dto::{NovaReasoningParams, NovaRequest};

pub fn build_headers(api_key: &str) -> Result<HeaderMap, reqwest::header::InvalidHeaderValue> {
    let mut headers = HeaderMap::new();
    let token = format!("Bearer {api_key}");
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}

pub fn create_request(
    ref_id: Option<String>,
    input: String,
    model: &str,
    verbosity: &str,
    max_tokens: u32,
    reasoning: &ReasoningSettings,
) -> NovaRequest {
    let reasoning_params = if reasoning.enabled {
        Some(NovaReasoningParams {
            effort: reasoning.effort.clone(),
        })
    } else {
        None
    };

    NovaRequest {
        ref_id,
        input,
        model: model.to_string(),
        verbosity: verbosity.to_string(),
        max_tokens,
        reasoning: reasoning.enabled,
        reasoning_params,
        image_urls: None,
    }
}
