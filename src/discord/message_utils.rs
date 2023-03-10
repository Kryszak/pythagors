use serenity::model::prelude::Message;

pub fn is_from_user(message: &Message) -> bool {
    return !message.author.bot;
}
