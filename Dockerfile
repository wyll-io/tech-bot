FROM rust:1 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/tech-bot /tech-bot

ENV DB_PATH=/app/tech-bot.db

WORKDIR /app

VOLUME [ "/app" ]

CMD [ "/tech-bot" ]