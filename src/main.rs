mod discord;
mod globals;
mod verification;

use std::sync::Arc;

use anyhow::anyhow;
use globals::Globals;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use verification::MessageVerificator;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("CLIENT_TOKEN") {
        token
    } else {
        return Err(anyhow!("'CLIENT_TOKEN' was not found").into());
    };

    let globals = Arc::new(Globals::new(secret_store));
    let message_verificator = MessageVerificator::new(globals);
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(message_verificator)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
