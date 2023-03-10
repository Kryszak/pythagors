use std::env;

#[derive(Clone)]
pub struct Globals {
    pub client_token: String,
    pub watched_channel: String,
    pub message_fetch_limit: i32,
    pub wrong_number_message_template: String,
    pub wrong_format_message_template: String,
}

impl Globals {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Globals {
            client_token: env::var("CLIENT_TOKEN").expect("No client token provided"),
            watched_channel: env::var("WATCHED_CHANNEL")
                .expect("No channel name provided to watch"),
            message_fetch_limit: env::var("MESSAGE_FETCH_LIMIT")
                .unwrap_or_else(|_| String::from("50"))
                .parse::<i32>()
                .expect("Failed to parse message limit count"),
            wrong_number_message_template: env::var("WRONG_INCREMENT_MESSAGE_TEMPLATE")
                .expect("No message template for wrong number posted provided"),
            wrong_format_message_template: env::var("WRONG_FORMAT_MESSAGE_TEMPLATE")
                .expect("No message template for wrong format posted provided"),
        }
    }
}
