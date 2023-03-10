mod globals;
mod verification;
mod discord;

use globals::Globals;
use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};
use tracing_subscriber::fmt;
use verification::MessageVerificator;

use tracing::error;

#[tokio::main]
async fn main() {
    let format = fmt::format()
        .with_target(false)
        .with_thread_ids(true)
        .with_ansi(true)
        .compact();
    tracing_subscriber::fmt().event_format(format).init();

    let globals = Globals::new();
    let token = globals.client_token.clone();

    let framework = StandardFramework::new();
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(MessageVerificator::new(globals))
        .await
        .expect("Err creating client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
