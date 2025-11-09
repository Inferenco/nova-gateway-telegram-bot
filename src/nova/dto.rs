use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NovaRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    pub input: String,
    pub model: String,
    pub verbosity: String,
    pub max_tokens: u32,
    pub reasoning: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_params: Option<NovaReasoningParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NovaReasoningParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NovaResponse {
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NovaErrorResponse {
    pub message: Option<String>,
}
