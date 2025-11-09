use std::collections::HashMap;

use teloxide::{types::{ChatId, Message}, Bot, RequestError};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{
    config::Config,
    nova::{helpers as nova_helpers, NovaClient, NovaClientError},
    utils,
};

use super::{dto::{BotCommand, ChatState}, helpers};

pub struct BotController {
    bot: Bot,
    nova_client: NovaClient,
    config: Config,
    chat_states: Mutex<HashMap<i64, ChatState>>,
}

#[derive(Debug, Error)]
pub enum BotError {
    #[error("telegram request error: {0}")]
    Telegram(#[from] RequestError),
    #[error("nova gateway error: {0}")]
    Nova(#[from] NovaClientError),
    #[error("I can only process text messages right now.")]
    MissingMessageText,
}

impl BotController {
    pub fn new(bot: Bot, nova_client: NovaClient, config: Config) -> Self {
        Self {
            bot,
            nova_client,
            config,
            chat_states: Mutex::new(HashMap::new()),
        }
    }

    pub async fn handle_command(&self, message: &Message, command: BotCommand) -> Result<(), BotError> {
        let chat_id = message.chat.id;

        match command {
            BotCommand::Help => self.send_help(chat_id).await,
            BotCommand::Reset => self.reset_conversation(chat_id).await,
            BotCommand::Chat => {
                let text = helpers::extract_plain_text(message).ok_or(BotError::MissingMessageText)?;
                // Extract text after /chat command (handles both /chat and /chat@botname)
                let prompt = if let Some(space_idx) = text.find(' ') {
                    text[space_idx + 1..].to_string()
                } else {
                    // No space found, command has no arguments
                    return Err(BotError::MissingMessageText);
                };
                
                if prompt.trim().is_empty() {
                    return Err(BotError::MissingMessageText);
                }
                
                self.forward_to_nova(chat_id, prompt.trim().to_string()).await
            }
        }
    }

    pub async fn handle_text_message(&self, _message: &Message) -> Result<(), BotError> {
        // Regular text messages are ignored - only /chat command is processed
        Ok(())
    }

    pub async fn notify_error(&self, chat_id: ChatId, error: &BotError) -> Result<(), RequestError> {
        if matches!(error, BotError::Telegram(_)) {
            return Ok(());
        }
        let message = error.user_message();
        let fallback = "Something went wrong while handling your request. Please try again.";
        let text = message.unwrap_or_else(|| fallback.to_string());
        utils::send_error(&self.bot, chat_id, text).await
    }

    async fn send_help(&self, chat_id: ChatId) -> Result<(), BotError> {
        let help_text = helpers::format_help_text();
        utils::send_text(&self.bot, chat_id, help_text).await?;
        Ok(())
    }

    async fn reset_conversation(&self, chat_id: ChatId) -> Result<(), BotError> {
        let ref_id = self.ensure_ref_id(chat_id).await;
        self.nova_client
            .clear_history(Some(ref_id.clone()))
            .await?;

        {
            let mut states = self.chat_states.lock().await;
            if let Some(state) = states.get_mut(&chat_id.0) {
                state.ref_id = Some(ref_id);
            }
        }

        utils::send_text(&self.bot, chat_id, "Conversation context cleared.").await?;
        Ok(())
    }

    async fn forward_to_nova(&self, chat_id: ChatId, text: String) -> Result<(), BotError> {
        let ref_id = self.ensure_ref_id(chat_id).await;
        let request = nova_helpers::create_request(
            Some(ref_id.clone()),
            text,
            self.config.nova_model(),
            self.config.nova_verbosity(),
            self.config.nova_max_tokens(),
            self.config.reasoning(),
        );

        let response = self.nova_client.send_prompt(request).await?;
        let reply = helpers::format_nova_response(&response);
        utils::send_text(&self.bot, chat_id, reply).await?;
        Ok(())
    }

    async fn ensure_ref_id(&self, chat_id: ChatId) -> String {
        let mut states = self.chat_states.lock().await;
        let state = states.entry(chat_id.0).or_insert_with(ChatState::default);
        state
            .ref_id
            .get_or_insert_with(|| chat_id.0.to_string())
            .clone()
    }
}

impl BotError {
    fn user_message(&self) -> Option<String> {
        match self {
            BotError::Telegram(_) => None,
            BotError::Nova(err) => Some(format!("Nova Gateway error: {err}")),
            BotError::MissingMessageText => Some("Please provide a message after /chat. Example: /chat Hello, how are you?".to_string()),
        }
    }
}
