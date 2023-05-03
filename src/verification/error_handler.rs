use std::sync::Arc;

use serenity::{model::prelude::Message, prelude::Context};
use tracing::debug;

use crate::discord::message_deleter::delete_message;
use crate::discord::message_sender::MessageSender;

use super::message_error::MessageError;

pub struct ErrorHandler {
    message_sender: Arc<MessageSender>,
}

impl ErrorHandler {
    pub fn new(message_sender: Arc<MessageSender>) -> Self {
        ErrorHandler { message_sender }
    }

    pub async fn handle_error(&self, error: MessageError, msg: &Message, context: &Context) {
        match error {
            MessageError::WrongFormat => {
                debug!("Handling wrong format error.");
                self.handle_wrong_format_error(msg, context).await;
            }
            MessageError::WrongNumber => {
                debug!("Handling wrong number error.");
                self.handle_wrong_number_error(msg, context).await;
            }
        }
    }

    async fn handle_wrong_format_error(&self, msg: &Message, context: &Context) {
        self.message_sender
            .notify_wrong_message_format(msg, context)
            .await;
        delete_message(msg, &context.http).await;
    }

    async fn handle_wrong_number_error(&self, msg: &Message, context: &Context) {
        self.message_sender
            .notify_wrong_number_provided(msg, context)
            .await;
        delete_message(msg, &context.http).await;
    }
}
