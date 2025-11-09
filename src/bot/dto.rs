use teloxide::utils::command::BotCommands;

#[derive(Debug, Clone, BotCommands)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum BotCommand {
    #[command(description = "Show help information")]
    Help,
    #[command(description = "Reset the current conversation context")]
    Reset,
    #[command(description = "Chat with Nova Gateway")]
    Chat,
}

#[derive(Debug, Clone, Default)]
pub struct ChatState {
    pub ref_id: Option<String>,
}
