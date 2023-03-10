use serenity::{
    async_trait,
    model::prelude::{Activity, Message, Ready},
    prelude::{Context, EventHandler},
};
use tracing::log::info;

use crate::globals::Globals;

pub struct MessageVerificator {
    globals: Globals,
}

impl MessageVerificator {
    pub fn new(globals: Globals) -> Self {
        return MessageVerificator { globals };
    }

    fn is_sent_from_user(msg: &Message) -> bool {
        return !msg.author.bot;
    }
}

#[async_trait]
impl EventHandler for MessageVerificator {
    async fn ready(&self, context: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        context
            .set_activity(Activity::watching("grę w cyferki"))
            .await;
    }

    async fn message(&self, context: Context, msg: Message) {
        let channel_name = msg.channel_id.name(&context).await.unwrap();
        if channel_name == self.globals.watched_channel
            && MessageVerificator::is_sent_from_user(&msg)
        {
            info!(
                "Veryfing message={} sent to channel={} by={}",
                msg.content, channel_name, msg.author.name
            );
        }
    }
}
