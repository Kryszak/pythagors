use std::{collections::HashMap, fs};

use toml::Value;

#[derive(Clone, Debug)]
pub struct Globals {
    pub client_token: String,
    pub watched_channel: String,
    pub message_fetch_limit: i32,
    pub wrong_number_message_template: String,
    pub wrong_format_message_template: String,
    pub rank_won_message_template: String,
    pub game_over_message_template: String,
    pub ranks: HashMap<String, String>,
    pub gameover_number: i32,
}

impl Globals {
    pub fn new() -> Self {
        let secret_file = fs::read_to_string("Secrets.toml").unwrap();
        let map: HashMap<String, Value> = toml::from_str(&secret_file).unwrap();
        Globals {
            client_token: map
                .get("CLIENT_TOKEN")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No client token provided"),
            watched_channel: map
                .get("WATCHED_CHANNEL")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No channel name provided to watch"),
            message_fetch_limit: map
                .get("MESSAGE_FETCH_LIMIT")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .unwrap_or_else(|| String::from("50"))
                .parse::<i32>()
                .expect("Failed to parse message limit count"),
            wrong_number_message_template: map
                .get("WRONG_INCREMENT_MESSAGE_TEMPLATE")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No message template for wrong number posted provided"),
            wrong_format_message_template: map
                .get("WRONG_FORMAT_MESSAGE_TEMPLATE")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No message template for wrong format posted provided"),
            rank_won_message_template: map
                .get("RANK_WON_MESSAGE_TEMPLATE")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No message template for rank won provided"),
            game_over_message_template: map
                .get("GAME_OVER_MESSAGE_TEMPLATE")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No message template for game over provided"),
            ranks: map
                .get("RANKS")
                .map(|x| x.as_str().unwrap())
                .map(|data| serde_json::from_str(data).expect("failed to parse ranks json"))
                .expect("No prized numbers provided"),
            gameover_number: map
                .get("GAME_OVER_NUMBER")
                .map(|x| x.as_str().unwrap())
                .map(String::from)
                .expect("No game over number provided")
                .parse::<i32>()
                .expect("Failed to parse game over number"),
        }
    }
}
