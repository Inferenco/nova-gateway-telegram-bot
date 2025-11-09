mod bot;
mod config;
mod nova;
mod utils;

use std::sync::Arc;

type DynError = Box<dyn std::error::Error + Send + Sync>;

use teloxide::{
    dptree,
    dispatching::{HandlerExt, UpdateFilterExt},
    prelude::Requester,
    types::Update,
    utils::command::BotCommands,
    Bot,
};

use bot::{handle_command_update, handle_message_update, BotCommand, BotController};
use config::Config;
use nova::NovaClient;

#[tokio::main]
async fn main() -> Result<(), DynError> {
    let config = Config::from_env()?;

    let bot = Bot::new(config.telegram_bot_token().to_string());
    bot.set_my_commands(BotCommand::bot_commands()).await?;

    let nova_client = NovaClient::new(
        config.nova_api_key().to_string(),
        config.nova_base_url().to_string(),
        config.nova_timeout_seconds(),
    )?;

    let controller = Arc::new(BotController::new(bot.clone(), nova_client, config));

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<BotCommand>()
                .endpoint(handle_command_update),
        )
        .branch(Update::filter_message().endpoint(handle_message_update));

    let mut dispatcher = teloxide::dispatching::Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![controller])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    Ok(())
}
