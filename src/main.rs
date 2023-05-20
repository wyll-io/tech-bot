mod commands;
mod database;

use poise::serenity_prelude as serenity;

use commands::{add, add_auth_user, help, list, remove, search, MsgData};
use database::DB;
use polodb_core::Database;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help(), add(), list(), search(), remove(), add_auth_user()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(MsgData {})
            })
        });

    DB.get_or_init(|| {
        Database::open_file(std::env::var("DB_PATH").expect("missing DB_PATH"))
            .expect("failed to initialize database")
    });
    framework.run().await.unwrap();
}
