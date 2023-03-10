use serenity::{model::prelude::Message, prelude::Context};

use crate::{
    discord::{message_sender::MessageSender, role_adder::RoleAdder},
    globals::Globals,
};

pub struct PrizeManager {
    globals: Globals,
    message_sender: MessageSender,
}

impl PrizeManager {
    pub fn new(globals: Globals) -> Self {
        PrizeManager {
            globals: globals.clone(),
            message_sender: MessageSender::new(globals),
        }
    }

    pub async fn add_role_for_prized_number(&self, msg: &Message, context: &Context, number: i32) {
        let role_id_for_number = self.globals.ranks.get(number.to_string());
        if let Some(role_id) = role_id_for_number {
            RoleAdder::add_role(msg, context, role_id.to_string()).await;
            self.message_sender
                .notify_rank_won(msg, context, role_id.to_string())
                .await;
        }
    }
}
