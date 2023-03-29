use serenity::{
    model::prelude::{Message, RoleId},
    prelude::Context,
};
use tracing::log::error;

pub async fn add_role(msg: &Message, context: &Context, role_id: String) {
    let http = &context.http;
    let mut user = msg.member(http).await.unwrap();
    if user
        .add_role(http, RoleId::from(role_id.parse::<u64>().unwrap()))
        .await
        .is_err()
    {
        error!(
            "Failed to add role={} to user={}",
            role_id,
            user.display_name()
        );
    }
}
