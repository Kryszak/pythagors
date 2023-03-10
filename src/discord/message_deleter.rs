use serenity::{http::Http, model::prelude::Message};
use tracing::log::{error, info};

pub struct MessageDeleter;

impl MessageDeleter {
    pub async fn delete_message(msg: &Message, http: &Http) {
        info!("Removing message from {}: {}", msg.author.name, msg.content);
        if let Err(_) = msg.delete(http).await {
            error!("Failed to delete message with content {}", msg.content);
        } else {
            info!(
                "Successfully removed message={} from={}",
                msg.content, msg.author.name
            );
        }
    }
}
