mod discord;
mod globals;
mod verification;

use globals::Globals;
use serenity::prelude::*;
use std::sync::Arc;
use tokio::signal;
use tracing::error;
use verification::MessageVerificator;

async fn shutdown_signal() {
    // Ctrl+C
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(false)
        .with_target(false)
        .with_level(true)
        .without_time()
        .init();

    let globals = Arc::new(Globals::new());
    let token = globals.client_token.clone();
    let message_verificator = MessageVerificator::new(globals);
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(message_verificator)
        .await
        .expect("Err creating client");

    // Spawn shutdown listener
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        shutdown_signal().await;
        println!("Shutdown signal received. Shutting down...");
        shard_manager.shutdown_all().await
    });

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
