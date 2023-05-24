use poise::command;
use url::Url;

use crate::database::*;

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

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, MsgData, Error>;

pub struct MsgData {}

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
    if !Url::parse(&link).is_ok() {
        ctx.say(format!("Link {link} is not a valid URL")).await?;
        return Ok(());
    }

    add_tech(link.clone(), technology.clone())?;

    ctx.say(format!("Added {technology} with link {link}",))
        .await?;

    Ok(())
}

/// List all available technologies
#[command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let techs = list_tech()?;
    if techs.len() == 0 {
        ctx.say("No technologies saved").await?;
        return Ok(());
    }

    ctx.say(format!(
        "Saved technologies: {}",
        techs
            .iter()
            .map(|tech| format!("[{}]({})", tech.name, tech.link))
            .collect::<Vec<String>>()
            .join(", ")
    ))
    .await?;

    Ok(())
}

/// Search for a technology
#[command(slash_command, prefix_command)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Technology name (can be a regex string)"] technology: String,
    #[description = "Regex options"] options: Option<String>,
) -> Result<(), Error> {
    let found_techs = search_tech(technology, options.map_or(String::new(), |opts| opts))?;
    if found_techs.len() == 0 {
        ctx.say("No technologies found").await?;
        return Ok(());
    }

    ctx.say(format!(
        "Found technologies: {}",
        found_techs
            .iter()
            .map(|tech| format!("Name: {} => Link: {}", tech.name, tech.link))
            .collect::<Vec<String>>()
            .join("\n")
    ))
    .await?;

    Ok(())
}

/// Remove a technology from the technologies list
#[command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Technology name"] technology: String,
) -> Result<(), Error> {
    if is_auth_user(ctx.author().id.to_string())? {
        ctx.say("You don't have permission to remove a technology")
            .await?;
        return Ok(());
    }

    remove_tech(technology)?;

    ctx.say("Technology removed").await?;

    Ok(())
}

/// Remove a technology from the technologies list
#[command(slash_command, prefix_command)]
pub async fn add_auth_user(
    ctx: Context<'_>,
    #[description = "Discord user ID"] id: String,
) -> Result<(), Error> {
    if !std::env::var("ADMIN_USERS")
        .expect("missing ADMIN_USERS")
        .contains(&ctx.author().id.to_string())
    {
        ctx.say("You don't have permission to add a new user")
            .await?;
        return Ok(());
    }

    set_auth_user(id)?;

    ctx.say("User added!").await?;

    Ok(())
}
