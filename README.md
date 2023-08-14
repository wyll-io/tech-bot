# tech-bot

## Description

Save technologies you want to share/remember in a simple way inside Discord.

## How to use

### Install

#### Docker

```bash
# BOT
cat << EOF > .env
DISCORD_TOKEN=TOKEN
GRAPHQL_ENDPOINT="http://database/graphql"
EOF

# DATABASE
cat << EOF > .env
DB_PATH="/path/to/db/database.db"
EOF

# FRONTEND
cat << EOF > $DOTENV_CONFIG_PATH # <==== Default to "/app/config/.env" in the image
PORT=3000
HOST=127.0.0.1
PUBLIC_GRAPHQL_ENDPOINT="http://database/graphql"
CLIENT_ID="DISCORD_APP_ID"
CLIENT_SECRET="DISCORD_APP_SECRET"
ORIGIN="http://host"
EOF

# Side note: ".env" file for "bot" and "database" are not mandatory, but if you don't use them, you will need to pass the environment variables to the container. Default path for ".env" is "/.env"

docker run -d \
  --name tech-bot-bot \
  -v $PWD/.env:/.env \
  --restart unless-stopped \
  gitea.antoine-langlois.net/datahearth/tech-bot:bot-latest

docker run -d \
  --name tech-bot-database \
  -v $PWD/.env:/.env \
  -v $DATABASE_VOLUME:/app \
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
# BOT
cat << EOF > .env
DISCORD_TOKEN=TOKEN
GRAPHQL_ENDPOINT="http://database/graphql"
EOF

# DATABASE
cat << EOF > .env
DB_PATH="/path/to/db/database.db"
EOF

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
export PUBLIC_GRAPHQL_ENDPOINT="http://database/graphql"
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

/search <technology> <options> <tags>


To remove a technology, you need to have the permission to remote a tech from the list.
If so, just type:

/remove <technology>


To update a technology, you need to have the permission to update a tech from the list.
If so, just type:

/update <UUID> <technology> <link> <tags>


To get help, just type:

/help

```
