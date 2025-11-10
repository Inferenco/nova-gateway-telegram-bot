use std::time::Duration;

use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, ChatId},
    Bot, RequestError,
};
use tokio::sync::oneshot;

pub async fn send_text(bot: &Bot, chat_id: ChatId, text: impl Into<String>) -> Result<(), RequestError> {
    bot.send_message(chat_id, text.into())
        .disable_web_page_preview(true)
        .await?;
    Ok(())
}

pub async fn send_error(bot: &Bot, chat_id: ChatId, text: impl Into<String>) -> Result<(), RequestError> {
    send_text(bot, chat_id, text).await
}

pub struct TypingIndicator {
    stop_signal: Option<oneshot::Sender<()>>,
}

impl TypingIndicator {
    pub fn start(bot: Bot, chat_id: ChatId) -> Self {
        let (stop_tx, mut stop_rx) = oneshot::channel();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut stop_rx => break,
                    _ = async {
                        let _ = bot.send_chat_action(chat_id, ChatAction::Typing).await;
                        tokio::time::sleep(Duration::from_secs(4)).await;
                    } => {}
                }
            }
        });

        Self {
            stop_signal: Some(stop_tx),
        }
    }
}

impl Drop for TypingIndicator {
    fn drop(&mut self) {
        if let Some(stop_signal) = self.stop_signal.take() {
            let _ = stop_signal.send(());
        }
    }
}
