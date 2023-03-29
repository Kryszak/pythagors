use serenity::{http::Http, model::prelude::Message};
use tracing::log::{error, info};

pub async fn delete_message(msg: &Message, http: &Http) {
    info!("Removing message from {}: {}", msg.author.name, msg.content);
    if msg.delete(http).await.is_err() {
        error!("Failed to delete message with content {}", msg.content);
    } else {
        info!(
            "Successfully removed message={} from={}",
            msg.content, msg.author.name
        );
    }
}
