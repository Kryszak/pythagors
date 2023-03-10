![Build Result](https://github.com/Kryszak/pythagors/actions/workflows/test.yml/badge.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/1dc1711a57c1423e81b6ffb027b61dd7)](https://www.codacy.com/gh/Kryszak/pythagors/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=Kryszak/pythagors&amp;utm_campaign=Badge_Grade)

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
### Config
In main project directory create `.env` file with following contents:
```bash
CLIENT_TOKEN=<bot's token>
WATCHED_CHANNEL=test1=<name of channel to watch>
WRONG_INCREMENT_MESSAGE_TEMPLATE==<Content of message sent, when user posts wrong number, e.g. '{{author}} learn learn to count'>
WRONG_FORMAT_MESSAGE_TEMPLATE==<Content of message sent, when user posts message in wrong format, 
e.g. '{{author}} read game rules - message was not correct'>
RANK_WON_MESSAGE_TEMPLATE=<Content of message sent, when user posts message with number winning role e.g. '{{author}}, congratulations on winning rank {{role}}!',>
RANKS=<JSON with number - rankId entries, e.g. {"10": "973271221112291409", "15": "973282436047839262"}> 
```
### Config placeholders
- `{{author}}` will be substituted with mention to message's author
- `{{role}}` will be substituted with mention to won role
