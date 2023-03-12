mod discord;
mod globals;
mod verification;

use anyhow::anyhow;
use globals::Globals;
use serenity::{prelude::GatewayIntents, Client};
use serenity_ctrlc::Ext;
use shuttle_secrets::SecretStore;
use verification::MessageVerificator;

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("CLIENT_TOKEN") {
        token
    } else {
        return Err(anyhow!("'CLIENT_TOKEN' was not found").into());
    };

    let globals = Globals::new(secret_store);
    let message_verificator = MessageVerificator::new(globals);
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(message_verificator)
        .await
        .expect("Err creating client")
        .ctrlc()
        .expect("Failed to register ctrl-c signal handler");

    Ok(client)
}
