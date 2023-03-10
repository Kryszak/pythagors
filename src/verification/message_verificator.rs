use serenity::{
    async_trait,
    model::prelude::{Activity, Message, Ready},
    prelude::{Context, EventHandler},
};
use tracing::log::info;

use crate::{
    discord::{message_fetcher::MessageFetcher, message_utils},
    globals::Globals,
};

use super::{error_handler::ErrorHandler, message_error::MessageError};

pub struct MessageVerificator {
    message_fetcher: MessageFetcher,
    globals: Globals,
    error_handler: ErrorHandler,
}

impl MessageVerificator {
    pub fn new(globals: Globals) -> Self {
        return MessageVerificator {
            message_fetcher: MessageFetcher::new(),
            globals,
            error_handler: ErrorHandler::new(),
        };
    }

    async fn verify_message(msg: &Message, context: Context) -> Result<(), MessageError> {
        // TODO run message verifications
        Ok(())
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
        if channel_name == self.globals.watched_channel && message_utils::is_from_user(&msg) {
            info!(
                "Veryfing message={} sent to channel={} by={}",
                msg.content, channel_name, msg.author.name
            );

            let messages = self
                .message_fetcher
                .get_last_messages(&msg, &context.http)
                .await
                .unwrap()
                .iter()
                .for_each(|m| info!("content: {}", m.content));

            match MessageVerificator::verify_message(&msg, context).await {
                Ok(_) => info!(
                    "Finished verification of message={} from={}",
                    msg.content, msg.author.name
                ),
                Err(error) => {
                    self.error_handler.handle_error(error).await;
                }
            }
        }
    }
}
