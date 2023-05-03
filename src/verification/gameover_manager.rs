use serenity::{
    model::{
        prelude::{Message, PermissionOverwrite, PermissionOverwriteType},
        Permissions,
    },
    prelude::Context,
};
use tracing::error;

use crate::{discord::message_sender::MessageSender, globals::Globals};

pub struct GameoverManager {
    globals: Globals,
    message_sender: MessageSender,
}

impl GameoverManager {
    pub fn new(globals: Globals) -> Self {
        GameoverManager {
            globals: globals.clone(),
            message_sender: MessageSender::new(globals),
        }
    }

    pub async fn check_for_game_over(&self, msg: &Message, context: &Context, number: i32) {
        if number == self.globals.gameover_number {
            self.message_sender.notify_game_over(msg, context).await;
            let everyone = msg
                .guild(context)
                .unwrap()
                .role_by_name("@everyone")
                .unwrap()
                .id;
            let write_locked = PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role(everyone),
            };
            if msg
                .channel_id
                .create_permission(&context.http, &write_locked)
                .await
                .is_err()
            {
                error!("Error while locking channel");
            }
        }
    }
}
