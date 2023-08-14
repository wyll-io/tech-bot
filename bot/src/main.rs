mod commands;
mod graphql;

use poise::serenity_prelude as serenity;

use commands::{add, help, list, remove, search, update, MsgData};
use reqwest::Client;

#[tokio::main]
async fn main() {
    match dotenvy::from_path(std::env::var("DOTENV_PATH").unwrap_or_else(|_| ".env".to_string())) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to load .env file: {}", e);
        }
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help(), add(), list(), search(), remove(), update()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(MsgData {
                    client: Client::new(),
                })
            })
        });

    framework.run().await.unwrap();
}
