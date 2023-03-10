use serenity::{http::Http, model::prelude::Message, Error};

use super::message_utils::is_from_user;

pub struct MessageFetcher;

impl MessageFetcher {
    pub fn new() -> Self {
        return MessageFetcher {};
    }

    pub async fn get_last_messages(&self, msg: &Message, http: &Http) -> Result<Vec<Message>, Error> {
        let messages = msg
            .channel_id
            .messages(http, |retriever| retriever.before(msg.id).limit(50))
            .await?;

        return Ok(messages.into_iter().filter(|m| is_from_user(m)).collect());
    }
}
