use std::collections::HashMap;

use crate::globals::Globals;
use serenity::{model::prelude::Message, prelude::Context};
use string_template::Template;
use tracing::log::error;

pub struct MessageSender {
    globals: Globals,
}

impl MessageSender {
    pub fn new(globals: Globals) -> Self {
        MessageSender { globals }
    }

    pub async fn notify_wrong_number_provided(&self, msg: &Message, context: &Context) {
        let msg_template = Template::new(&self.globals.wrong_number_message_template);
        let mut parameters: HashMap<&str, &str> = HashMap::new();
        let author_mention = format!("<@{}>", msg.author.id);
        parameters.insert("author", &author_mention);
        let message_content = msg_template.render(&parameters);

        MessageSender::send(msg, context, message_content).await;
    }

    pub async fn notify_wrong_message_format(&self, msg: &Message, context: &Context) {
        let msg_template = Template::new(&self.globals.wrong_format_message_template);
        let mut parameters: HashMap<&str, &str> = HashMap::new();
        let author_mention = format!("<@{}>", msg.author.id);
        parameters.insert("author", &author_mention);
        let message_content = msg_template.render(&parameters);

        MessageSender::send(msg, context, message_content).await;
    }

    pub async fn notify_rank_won(&self, msg: &Message, context: &Context, role_id: String) {
        let msg_template = Template::new(&self.globals.rank_won_message_template);
        let mut parameters: HashMap<&str, &str> = HashMap::new();
        let author_mention = format!("<@{}>", msg.author.id);
        let role_mention = format!("<@&{}>", role_id);
        parameters.insert("author", &author_mention);
        parameters.insert("role", &role_mention);
        let message_content = msg_template.render(&parameters);

        MessageSender::send(msg, context, message_content).await;
    }

    async fn send(original_msg: &Message, context: &Context, content: String) {
        if original_msg
            .channel_id
            .send_message(&context.http, |message| message.content(content))
            .await
            .is_err()
        {
            error!(
                "Failed to send message to channel={}",
                original_msg.channel_id
            );
        }
    }
}
