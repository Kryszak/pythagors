use serenity::model::prelude::Message;

pub fn is_from_user(message: &Message) -> bool {
    return !message.author.bot;
}

pub fn extract_number_from_message(msg: &Message) -> Option<i32> {
    return msg
        .content
        .split(" ")
        .next()
        .map(String::from)
        .and_then(|s| try_parse(s));
}

pub fn contains_number(msg: &Message) -> bool {
    return extract_number_from_message(msg).is_some();
}

fn try_parse(value: String) -> Option<i32> {
    return value.parse::<i32>().ok();
}
