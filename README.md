![Build Result](https://github.com/Kryszak/pythagors/actions/workflows/build.yml/badge.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/1dc1711a57c1423e81b6ffb027b61dd7)](https://app.codacy.com/gh/Kryszak/pythagors/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

# Pythagors
Rewrite of [cyferki-watcher-bot](https://github.com/Kryszak/cyferki-watcher-bot) in Rust to leverage 
[shuttle](https://www.shuttle.rs/) deployments.

## Rules
Game rules can be found in [Rules](./RULES.md)

## Required bot Privileged Gateway Intents
- Message Content Intent

## Required bot user permissions
- Manage Roles :warning: Bot's role must be higher than any role that it sets for this feature to work ⚠️
- Read Messages/View Channels
- Send Messages
- Manage Messages

## local dev config
### Tools
Make sure that rust toolset is up to date.
```bash
rustup update
```
Shuttle plugin for cargo is required for building, running and deployment of bot
```bash
cargo install cargo-shuttle
```
### Config
In main project directory create `Secrets.toml` file with following contents:
```toml
CLIENT_TOKEN="" # bot token
WATCHED_CHANNEL="test1" # name of channel to watch
WRONG_INCREMENT_MESSAGE_TEMPLATE="{{author}} learn learn to count" # Content of message sent, when user posts wrong number
WRONG_FORMAT_MESSAGE_TEMPLATE="{{author}} read game rules - message was not correct" # Content of message sent, when user posts message in wrong format
RANK_WON_MESSAGE_TEMPLATE="{{author}}, congratulations on winning rank {{role}}!" # Content of message sent, when user posts message with number winning role 
GAME_OVER_MESSAGE_TEMPLATE="Game over! Thanks for playing" # Content of message sent on last number
RANKS='{"10": "973271221112291409", "15": "973282436047839262"}' # JSON with number - rankId entries
GAME_OVER_NUMBER="16" # Number, on which game will end as a string
```
### Config placeholders
- `{{author}}` will be substituted with mention to message's author
- `{{role}}` will be substituted with mention to won role

### Run project locally
```bash
cargo shuttle run
```

### Deploy to shuttle
```bash
cargo shuttle project new # only if project does not exist yet
cargo shuttle project start
cargo shuttle deploy
```
Secrets are populated initially from `Secrets.toml` file and can be edited with `cargo shuttle secrets` command.

### Stop deployment
```bash
cargo shuttle project stop
```

