use std::borrow::ToOwned;

use teloxide::types::Message;

use crate::nova::NovaResponse;

pub fn extract_plain_text(message: &Message) -> Option<String> {
    message.text().map(ToOwned::to_owned)
}

pub fn format_help_text() -> String {
    vec![
        "Hello! I'm a Nova Gateway assistant.",
        "\nUse these commands:",
        "/start - Start interacting with the bot",
        "/help - Show this help message",
        "/reset - Clear the conversation context",
        "\nSend any other message and I'll forward it to Nova Gateway.",
    ]
    .join("\n")
}

pub fn format_nova_response(response: &NovaResponse) -> String {
    if let Some(text) = response.text.clone() {
        if !text.trim().is_empty() {
            return text;
        }
    }

    "Nova Gateway returned an empty response.".to_string()
}
