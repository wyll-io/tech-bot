# ********************************
# * Build the bot and database *
# ********************************

FROM rust:1 as rust-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bot /app/bot
COPY database /app/database

RUN cargo build -r --workspace

# ********************************
# * Deploy the bot *
# ********************************

FROM gcr.io/distroless/cc as bot

COPY --from=rust-builder /app/target/release/bot /bot

CMD [ "/bot" ]

# ********************************
# * Deploy the database *
# ********************************

FROM gcr.io/distroless/cc as database

COPY --from=rust-builder /app/target/release/database /database

ENV DB_PATH=/app/tech-bot.db

WORKDIR /app

VOLUME [ "/app" ]

CMD [ "/database" ]

# ********************************
# * Build the web *
# ********************************

FROM node:18-slim as web-builder

WORKDIR /app

COPY frontend/ ./

RUN npm install
RUN npm run build

FROM node:18-alpine3.18 as web

WORKDIR /app

COPY --from=web-builder /app/build ./build 
COPY --from=web-builder /app/node_modules ./node_modules
COPY --from=web-builder /app/package.json /app/package-lock.json ./

ENV DOTENV_CONFIG_PATH=/app/config/.env

VOLUME [ "/app/config" ]

CMD [ "node", "-r", "dotenv/config", "build" ]