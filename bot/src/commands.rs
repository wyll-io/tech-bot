use std::env;

use graphql_client::{GraphQLQuery, Response};
use poise::command;
use reqwest::Client;
use url::Url;
use uuid::Uuid;

use crate::graphql::{
    create_technology, delete_technology, get_technologies, get_technology, update_technology,
    CreateTechnology, DeleteTechnology, GetTechnologies, GetTechnology, UpdateTechnology,
};

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

To update a technology, you need to have the permission to update a tech from the list.
If so, just type:
```
/update <UUID> <technology> <link> <tags>
```

To get help, just type:
```
/help
```
";

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, MsgData, Error>;

pub struct MsgData {
    pub client: Client,
}

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
    #[description = "Technology tags (comma separated)"] tags: Option<String>,
) -> Result<(), Error> {
    if !Url::parse(&link).is_ok() {
        ctx.say(format!("Link {link} is not a valid URL")).await?;
        return Ok(());
    }

    let tags = tags.unwrap_or(String::new());
    let final_tags = tags.split(',').map(|s| s.to_string());

    let rsp: Response<create_technology::ResponseData> = ctx
        .data()
        .client
        .post(env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT not set"))
        .json(&CreateTechnology::build_query(
            create_technology::Variables {
                name: technology,
                link,
                tags: final_tags.collect::<Vec<String>>(),
                user_id: ctx.author().id.to_string(),
            },
        ))
        .send()
        .await?
        .json()
        .await?;

    if let Some(errors) = rsp.errors {
        ctx.say(format!("command failed: {:?}", errors)).await?;
        return Ok(());
    }

    let data = rsp.data.unwrap().create_technology;

    ctx.say(format!(
        "Added {} with link {} and tags [{}] (id: {})",
        data.name,
        data.link,
        data.tags.join(","),
        data.id
    ))
    .await?;

    Ok(())
}

/// List all available technologies
#[command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let rsp: Response<get_technologies::ResponseData> = ctx
        .data()
        .client
        .post(env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT not set"))
        .json(&GetTechnologies::build_query(
            get_technologies::Variables {},
        ))
        .send()
        .await?
        .json()
        .await?;

    if let Some(errors) = rsp.errors {
        ctx.say(format!("command failed: {:?}", errors)).await?;
        return Ok(());
    }

    let techs = rsp.data.unwrap().technologies;

    if techs.len() == 0 {
        ctx.say("No technologies saved").await?;
        return Ok(());
    }

    ctx.say(format!(
        "Saved technologies: \n{}",
        techs
            .iter()
            .map(|tech| format!(
                "[{}]({}) **{}** (id: {})",
                tech.name,
                tech.link,
                tech.tags.join(","),
                tech.id
            ))
            .collect::<Vec<String>>()
            .join("\n")
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
    #[description = "Technology tags (comma separated)"] tags: Option<String>,
) -> Result<(), Error> {
    let tags = tags.unwrap_or(String::new());
    let final_tags = tags.split(',').map(|s| s.to_string());

    let rsp: Response<get_technology::ResponseData> = ctx
        .data()
        .client
        .post(env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT not set"))
        .json(&GetTechnology::build_query(get_technology::Variables {
            name: technology,
            options: options.unwrap_or(String::new()),
            tags: final_tags.collect::<Vec<String>>(),
        }))
        .send()
        .await?
        .json()
        .await?;

    if let Some(errors) = rsp.errors {
        ctx.say(format!("command failed: {:?}", errors)).await?;
        return Ok(());
    }

    let found_techs = rsp.data.unwrap().technology;

    if found_techs.len() == 0 {
        ctx.say("No technologies found").await?;
        return Ok(());
    }

    ctx.say(format!(
        "Found technologies: {}",
        found_techs
            .iter()
            .map(|tech| format!("\nName: {} => Link: {}", tech.name, tech.link))
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
    #[description = "Technology name"] id: Uuid,
) -> Result<(), Error> {
    let rsp: Response<delete_technology::ResponseData> = ctx
        .data()
        .client
        .post(env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT not set"))
        .json(&DeleteTechnology::build_query(
            delete_technology::Variables { id },
        ))
        .send()
        .await?
        .json()
        .await?;

    if let Some(errors) = rsp.errors {
        ctx.say(format!("command failed: {:?}", errors)).await?;
        return Ok(());
    }

    ctx.say("Technology removed").await?;

    Ok(())
}

// Update a technology
#[command(slash_command, prefix_command)]
pub async fn update(
    ctx: Context<'_>,
    #[description = "Technology UUID"] id: Uuid,
    #[description = "New name"] name: Option<String>,
    #[description = "New link"] link: Option<String>,
    #[description = "New tags"] tags: Option<String>,
) -> Result<(), Error> {
    if name.is_none() && link.is_none() && tags.is_none() {
        ctx.say("Nothing to update").await?;
        return Ok(());
    }
    let mut final_tags = None;

    if let Some(tags) = tags {
        let res = tags
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        final_tags = Some(res);
    }

    let rsp: Response<update_technology::ResponseData> = ctx
        .data()
        .client
        .post(env::var("GRAPHQL_ENDPOINT").expect("GRAPHQL_ENDPOINT not set"))
        .json(&UpdateTechnology::build_query(
            update_technology::Variables {
                id,
                name,
                link,
                tags: final_tags,
            },
        ))
        .send()
        .await?
        .json()
        .await?;

    if let Some(errors) = rsp.errors {
        ctx.say(format!("command failed: {:?}", errors)).await?;
        return Ok(());
    }

    ctx.say("Technology updated").await?;

    Ok(())
}
