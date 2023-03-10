use serenity::{http::Http, model::prelude::Message, Error};

use super::message_utils::is_from_user;

pub struct MessageFetcher {
    message_limit: u64,
}

impl MessageFetcher {
    pub fn new(limit: i32) -> Self {
        MessageFetcher {
            message_limit: limit as u64,
        }
    }

    pub async fn get_last_messages(
        &self,
        msg: &Message,
        http: &Http,
    ) -> Result<Vec<Message>, Error> {
        let messages = msg
            .channel_id
            .messages(http, |retriever| {
                retriever.before(msg.id).limit(self.message_limit)
            })
            .await?;

        Ok(messages.into_iter().filter(is_from_user).collect())
    }
}
