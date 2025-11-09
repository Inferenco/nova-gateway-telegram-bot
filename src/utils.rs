use teloxide::{payloads::SendMessageSetters, prelude::Requester, types::ChatId, Bot, RequestError};

pub async fn send_text(bot: &Bot, chat_id: ChatId, text: impl Into<String>) -> Result<(), RequestError> {
    bot.send_message(chat_id, text.into())
        .disable_web_page_preview(true)
        .await?;
    Ok(())
}

pub async fn send_error(bot: &Bot, chat_id: ChatId, text: impl Into<String>) -> Result<(), RequestError> {
    send_text(bot, chat_id, text).await
}
