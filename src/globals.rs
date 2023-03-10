use std::env;

#[derive(Clone)]
pub struct Globals {
    pub client_token: String,
    pub watched_channel: String,
}

impl Globals {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        return Globals {
            client_token: env::var("CLIENT_TOKEN").expect("No client token provided"),
            watched_channel: env::var("WATCHED_CHANNEL").expect("No channel name provided to watch"),
        };
    }
}
