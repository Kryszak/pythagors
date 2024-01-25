use serenity::{builder::GetMessages, http::Http, model::prelude::Message, Error};

use super::message_utils::is_from_user;

pub struct MessageFetcher {
    message_limit: u8,
}

impl MessageFetcher {
    pub fn new(limit: i32) -> Self {
        MessageFetcher {
            message_limit: limit as u8,
        }
    }

    pub async fn get_last_messages(
        &self,
        msg: &Message,
        http: &Http,
    ) -> Result<Vec<Message>, Error> {
        let message_filter = GetMessages::new().before(msg.id).limit(self.message_limit);
        let messages = msg.channel_id.messages(http, message_filter).await?;

        Ok(messages.into_iter().filter(is_from_user).collect())
    }
}
