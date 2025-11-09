use teloxide::utils::command::BotCommands;

#[derive(Debug, Clone, BotCommands)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum BotCommand {
    #[command(description = "Start interacting with the bot")]
    Start,
    #[command(description = "Show help information")]
    Help,
    #[command(description = "Reset the current conversation context")]
    Reset,
}

#[derive(Debug, Clone, Default)]
pub struct ChatState {
    pub ref_id: Option<String>,
}
