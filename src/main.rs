mod discord;
mod globals;
mod verification;

use globals::Globals;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use verification::MessageVerificator;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(false)
        .with_target(false)
        .with_level(true)
        .without_time()
        .init();

    let globals = Arc::new(Globals::new(secret_store));
    let token = globals.client_token.clone();
    let message_verificator = MessageVerificator::new(globals);
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(message_verificator)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
