use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};
use tracing::log::info;

pub struct MessageVerificator;

#[async_trait]
impl EventHandler for MessageVerificator {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, context: Context, msg: Message) {
        let channel_name = msg.channel_id.name(&context).await.unwrap();
        info!(
            "Veryfing message={} sent to channel {} by {}",
            msg.content, channel_name, msg.author.name
        );
    }
}
