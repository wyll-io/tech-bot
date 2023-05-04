use std::sync::Mutex;

use once_cell::sync::OnceCell;
use poise::command;

use crate::database::Database;

const HELP_MESSAGE: &str = "
Hello fellow human! I am a bot that can help you adding new technologies to a git repository.

To add a new technology, just type:
```
/add <technology> <link>
```

To list all technologies, just type:
```
/list
```

To search for a technology, just type:
```
/search <technology>
```

To remove a technology, you need to have the permission to remote a tech from the list.
If so, just type:
```
/remove <technology>
```

To get help, just type:
```
/help
```
";
const AUTHORIZED_USERS: [u64; 1] = [252497456447750144];

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, MsgData, Error>;

pub struct MsgData {}

pub static DB: OnceCell<Mutex<Database>> = OnceCell::new();

/// Show help for all commands
#[command(slash_command, prefix_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(HELP_MESSAGE).await?;

    Ok(())
}

/// Add a new technology to the technologies list
#[command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Technology name"] technology: String,
    #[description = "Git repository link"] link: String,
) -> Result<(), Error> {
    ctx.say(format!("Added {technology} with link {link}"))
        .await?;

    Ok(())
}

/// List all available technologies
#[command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Listed all technologies").await?;

    Ok(())
}

/// Search for a technology
#[command(slash_command, prefix_command)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Technology name"] technology: String,
) -> Result<(), Error> {
    ctx.say(format!("Found {technology}")).await?;

    Ok(())
}

/// Remove a technology from the technologies list
#[command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Technology name"] technology: String,
) -> Result<(), Error> {
    if !AUTHORIZED_USERS.contains(&ctx.author().id.0) {
        ctx.say("You don't have permission to remove a technology")
            .await?;
        return Ok(());
    }

    ctx.say(format!("Removed {technology}")).await?;

    Ok(())
}
