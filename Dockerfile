# ********************************
# * Build & deploy bot ***********
# ********************************
FROM rust:1 as bot-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock bot ./

RUN cargo build --release --bin bot


FROM gcr.io/distroless/cc as bot

COPY --from=bot-builder /app/target/release/bot /bot

CMD [ "/bot" ]

# ********************************
# * Build & deploy the database **
# ********************************
FROM rust:1 as db-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock database ./

RUN cargo build --release --bin database


FROM gcr.io/distroless/cc as database

COPY --from=db-builder /app/target/release/database /database

ENV DB_PATH=/app/tech-bot.db

WORKDIR /app

VOLUME [ "/app" ]

CMD [ "/database" ]

# ********************************
# * Build the web ***************
# ********************************

FROM node:18-slim as web-dependencies

WORKDIR /app

COPY frontend/package.json frontend/.npmrc ./

RUN npm install

FROM node:18-alpine3.18 as web

WORKDIR /tmp/app

COPY frontend/ ./

COPY --from=web-dependencies /app/node_modules ./node_modules

RUN npm run build

WORKDIR /app

RUN mv /tmp/app/build /tmp/app/node_modules /tmp/app/package.json .
RUN rm -rf /tmp/app

COPY --from=web-dependencies /app/package-lock.json ./

ENV DOTENV_CONFIG_PATH=/app/config/.env

VOLUME [ "/app/config" ]

CMD [ "node", "-r", "dotenv/config", "build" ]