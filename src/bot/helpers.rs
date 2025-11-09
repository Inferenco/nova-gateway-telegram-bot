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
        "/help - Show this help message",
        "/reset - Clear the conversation context",
        "/chat - Chat with Nova Gateway",
        "\nExample: /chat Hello, how are you?",
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
