mod commands;
mod database;

use poise::serenity_prelude as serenity;

use commands::{add, help, list, remove, search, MsgData, DB};

#[tokio::main]
async fn main() {
    DB.set(Mutex)

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help(), add(), list(), search(), remove()],
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

    framework.run().await.unwrap();
}
