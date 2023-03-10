use serenity::model::prelude::Message;

pub fn is_from_user(message: &Message) -> bool {
    !message.author.bot
}

pub fn extract_number_from_message(msg: &Message) -> Option<i32> {
    return msg
        .content
        .split(' ')
        .next()
        .map(String::from)
        .and_then(try_parse);
}

pub fn contains_number(msg: &Message) -> bool {
    extract_number_from_message(msg).is_some()
}

fn try_parse(value: String) -> Option<i32> {
    value.parse::<i32>().ok()
}
