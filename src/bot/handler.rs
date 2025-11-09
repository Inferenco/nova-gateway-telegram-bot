use std::sync::Arc;

use teloxide::{types::Message, RequestError};

use super::{controller::{BotController, BotError}, dto::BotCommand};

pub type HandlerResult = Result<(), RequestError>;

pub async fn handle_command_update(
    controller: Arc<BotController>,
    message: Message,
    command: BotCommand,
) -> HandlerResult {
    match controller.handle_command(&message, command).await {
        Ok(()) => Ok(()),
        Err(BotError::Telegram(err)) => Err(err),
        Err(other) => {
            controller.notify_error(message.chat.id, &other).await?;
            Ok(())
        }
    }
}

pub async fn handle_message_update(
    controller: Arc<BotController>,
    message: Message,
) -> HandlerResult {
    match controller.handle_text_message(&message).await {
        Ok(()) => Ok(()),
        Err(BotError::Telegram(err)) => Err(err),
        Err(other) => {
            controller.notify_error(message.chat.id, &other).await?;
            Ok(())
        }
    }
}
