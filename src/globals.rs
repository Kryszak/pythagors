use std::env;

pub struct Globals {
    pub client_token: String,
}

impl Globals {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        return Globals {
            client_token: env::var("CLIENT_TOKEN").expect("No client token provided"),
        };
    }
}
