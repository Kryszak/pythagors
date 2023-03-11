use serde_json::Value;
use shuttle_secrets::SecretStore;

#[derive(Clone)]
pub struct Globals {
    pub client_token: String,
    pub watched_channel: String,
    pub message_fetch_limit: i32,
    pub wrong_number_message_template: String,
    pub wrong_format_message_template: String,
    pub rank_won_message_template: String,
    pub ranks: Value,
}

impl Globals {
    pub fn new(secret_store: SecretStore) -> Self {
        Globals {
            client_token: secret_store
                .get("CLIENT_TOKEN")
                .expect("No client token provided"),
            watched_channel: secret_store
                .get("WATCHED_CHANNEL")
                .expect("No channel name provided to watch"),
            message_fetch_limit: secret_store
                .get("MESSAGE_FETCH_LIMIT")
                .unwrap_or_else(|| String::from("50"))
                .parse::<i32>()
                .expect("Failed to parse message limit count"),
            wrong_number_message_template: secret_store
                .get("WRONG_INCREMENT_MESSAGE_TEMPLATE")
                .expect("No message template for wrong number posted provided"),
            wrong_format_message_template: secret_store
                .get("WRONG_FORMAT_MESSAGE_TEMPLATE")
                .expect("No message template for wrong format posted provided"),
            rank_won_message_template: secret_store
                .get("RANK_WON_MESSAGE_TEMPLATE")
                .expect("No message template for rank won provided"),
            ranks: secret_store
                .get("RANKS")
                .map(|data| serde_json::from_str(&data).expect("failed to parse data"))
                .expect("No prized numbers provided"),
        }
    }
}
