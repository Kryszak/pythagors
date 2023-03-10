#!/bin/bash

SERVER_ID=972581166861398067
BOT_TOKEN=$(grep CLIENT_TOKEN ../.env | sed 's/CLIENT_TOKEN=//g')

curl -s https://discord.com/api/guilds/$SERVER_ID/roles -H "Authorization: Bot $BOT_TOKEN" | jq '.[] | "\(.id): \(.name)"'
