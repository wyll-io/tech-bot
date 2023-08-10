# tech-bot

## Description

Save technologies you want to share/remember in a simple way inside Discord.

## How to use

### Install

#### Docker

```bash
# BOT
export DISCORD_TOKEN=TOKEN
export GRAPHQL_ENDPOINT="http://host/graphql"

# DATABASE
export DB_PATH="/path/to/db/database.db"

# FRONTEND
export FRONT_VOLUME
cat << EOF > $FRONT_VOLUME/.env
PORT=3000
HOST=127.0.0.1
PUBLIC_GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT"
CLIENT_ID="DISCORD_APP_ID"
CLIENT_SECRET="DISCORD_APP_SECRET"
ORIGIN="http://host"
EOF

docker run -d \
  --name tech-bot-bot \
  -e DISCORD_TOKEN=$DISCORD_TOKEN \
  -e GRAPHQL_ENDPOINT=$GRAPHQL_ENDPOINT \
  --restart unless-stopped \
  gitea.antoine-langlois.net/datahearth/tech-bot:bot-latest

docker run -d \
  --name tech-bot-database \
  -v $DB_PATH:/app/database.db \
  --restart unless-stopped \
  gitea.antoine-langlois.net/datahearth/tech-bot:database-latest

docker run -d \
  --name tech-bot-front \
  -v $FRONT_VOLUME:/app/config \
  -p ${EXTERNAL_PORT}:${INTERNAL_PORT} \
  --restart unless-stopped \
  gitea.antoine-langlois.net/datahearth/tech-bot:front-latest
```

#### Manually

```bash
# Setup
git clone https://gitea.antoine-langlois.net/datahearth/tech-bot.git
cd tech-bot

# Database & Bot
## Build
cargo build --release

# Run
. ./target/release/tech-bot/database
. ./target/release/tech-bot/bot

# Frontend
cd frontend

## Build
pnpm install --prod --frozen-lockfile
pnpm build

## Run
export PORT=3000
export HOST=127.0.0.1
export PUBLIC_GRAPHQL_ENDPOINT="$GRAPHQL_ENDPOINT"
export CLIENT_ID="DISCORD_APP_ID"
export CLIENT_SECRET="DISCORD_APP_SECRET"
export ORIGIN="http://host"
node -r dotenv/config build
```

### Usage

```text
Hello fellow human! I am a bot that can help you adding new technologies to a git repository.

To add a new technology, just type:

/add <technology> <link>

To list all technologies, just type:

/list


To search for a technology, just type:

/search <technology>


To remove a technology, you need to have the permission to remote a tech from the list.
If so, just type:

/remove <technology>


To update a technology, you need to have the permission to update a tech from the list.
If so, just type:

/update <UUID> <technology> <link> <tags>


To get help, just type:

/help

```
