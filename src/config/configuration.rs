use std::env;

use dotenvy::dotenv;
use thiserror::Error;

use super::dto::ReasoningSettings;

const DEFAULT_BASE_URL: &str = "https://gateway.inferenco.com";
const DEFAULT_MODEL: &str = "gpt-5-mini";
const DEFAULT_VERBOSITY: &str = "Medium";
const DEFAULT_MAX_TOKENS: u32 = 1024;
const DEFAULT_TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Clone)]
pub struct Config {
    telegram_bot_token: String,
    nova_api_key: String,
    nova_base_url: String,
    nova_model: String,
    nova_verbosity: String,
    nova_max_tokens: u32,
    reasoning: ReasoningSettings,
    nova_timeout_seconds: u64,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing environment variable: {0}")]
    MissingVar(&'static str),
    #[error("invalid number for {0}: {1}")]
    InvalidNumber(&'static str, String),
    #[error("invalid boolean for {0}: {1}")]
    InvalidBoolean(&'static str, String),
    #[error("environment error: {0}")]
    Env(#[from] env::VarError),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let telegram_bot_token = load_required("TELEGRAM_BOT_TOKEN")?;
        let nova_api_key = load_required("NOVA_API_KEY")?;

        let nova_base_url = env::var("NOVA_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        let nova_model = env::var("NOVA_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        let nova_verbosity = env::var("NOVA_VERBOSITY").unwrap_or_else(|_| DEFAULT_VERBOSITY.to_string());

        let nova_max_tokens = match env::var("NOVA_MAX_TOKENS") {
            Ok(value) => value
                .parse::<u32>()
                .map_err(|_| ConfigError::InvalidNumber("NOVA_MAX_TOKENS", value))?,
            Err(_) => DEFAULT_MAX_TOKENS,
        };

        let reasoning_enabled = match env::var("NOVA_REASONING") {
            Ok(value) => parse_bool(&value).ok_or_else(|| ConfigError::InvalidBoolean("NOVA_REASONING", value))?,
            Err(_) => false,
        };

        let reasoning_effort = env::var("NOVA_REASONING_EFFORT").ok().filter(|value| !value.is_empty());

        let nova_timeout_seconds = match env::var("NOVA_TIMEOUT_SECONDS") {
            Ok(value) => value
                .parse::<u64>()
                .map_err(|_| ConfigError::InvalidNumber("NOVA_TIMEOUT_SECONDS", value))?,
            Err(_) => DEFAULT_TIMEOUT_SECS,
        };

        Ok(Self {
            telegram_bot_token,
            nova_api_key,
            nova_base_url,
            nova_model,
            nova_verbosity,
            nova_max_tokens,
            reasoning: ReasoningSettings {
                enabled: reasoning_enabled,
                effort: reasoning_effort,
            },
            nova_timeout_seconds,
        })
    }

    pub fn telegram_bot_token(&self) -> &str {
        &self.telegram_bot_token
    }

    pub fn nova_api_key(&self) -> &str {
        &self.nova_api_key
    }

    pub fn nova_base_url(&self) -> &str {
        &self.nova_base_url
    }

    pub fn nova_model(&self) -> &str {
        &self.nova_model
    }

    pub fn nova_verbosity(&self) -> &str {
        &self.nova_verbosity
    }

    pub fn nova_max_tokens(&self) -> u32 {
        self.nova_max_tokens
    }

    pub fn reasoning(&self) -> &ReasoningSettings {
        &self.reasoning
    }

    pub fn nova_timeout_seconds(&self) -> u64 {
        self.nova_timeout_seconds
    }
}

fn load_required(key: &'static str) -> Result<String, ConfigError> {
    env::var(key).map_err(|err| match err {
        env::VarError::NotPresent => ConfigError::MissingVar(key),
        other => ConfigError::Env(other),
    })
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Some(true),
        "false" | "0" | "no" | "n" => Some(false),
        _ => None,
    }
}
