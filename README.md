# tech-bot

## Description

Save technologies you want to share/remember in a simple way inside Discord.

## How to use

### Install

#### Docker

```bash
export DISCORD_TOKEN=TOKEN
export ADMIN_USERS=123456789,987654321

docker run -d \
  --name tech-bot \
  -e DISCORD_TOKEN=$DISCORD_TOKEN \
  -e ADMIN_USERS=$ADMIN_USERS \
  -v /path/to/tech-bot/data:/data \
  --restart unless-stopped \
  gitea.antoine-langlois.net/datahearth/tech-bot
```

#### Manually

```bash
export DISCORD_TOKEN=TOKEN
export ADMIN_USERS=123456789,987654321
export DB_PATH=database.db

git clone https://gitea.antoine-langlois.net/datahearth/tech-bot.git
cd tech-bot

cargo build --release
./target/release/tech-bot
```

### Usage

```bash
/tech help
```

```bash
/tech add <Technology name> <Technology link> <OPTIONAL: Technology tags (comma separated)>
```

```bash
/tech list
```

```bash
/tech search <Technology name> <OPTIONAL: Regex> <OPTIONAL: Technology tags (comma separated)>
```

```bash
/tech remove <tech-name>
```
