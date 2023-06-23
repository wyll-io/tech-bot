use once_cell::sync::OnceCell;
use polodb_core::{
    bson::{doc, Regex},
    Database,
};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

pub static DB: OnceCell<Database> = OnceCell::new();

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Technology {
    pub link: String,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizedUser {
    discord_id: String,
}

/// Add a new technology to the database.
pub fn add_tech(name: &str, link: &str, tags: &[&str]) -> Result<(), Error> {
    DB.get()
        .unwrap()
        .collection::<Technology>("technologies")
        .insert_one(Technology {
            link: link.into(),
            name: name.into(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
        })
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

    Ok(())
}

pub fn remove_tech(name: String) -> Result<(), Error> {
    DB.get()
        .unwrap()
        .collection::<Technology>("technologies")
        .delete_one(doc! { "name": name })
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

    Ok(())
}

pub fn list_tech() -> Result<Vec<Technology>, Error> {
    Ok(DB
        .get()
        .unwrap()
        .collection("technologies")
        .find(None)
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
        .filter(|doc| doc.is_ok())
        .map(|doc| doc.unwrap())
        .collect())
}

pub fn search_tech(name: String, options: String, tags: &[&str]) -> Result<Vec<Technology>, Error> {
    Ok(DB
        .get()
        .unwrap()
        .collection::<Technology>("technologies")
        .find(doc! { "name": {"$regex": Regex {
            pattern: name,
            options: options,
        }}, "tags": {
            "$in": tags
        } })
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
        .map(
            |doc| doc.unwrap(), // todo: find a way to handle error
        )
        .collect::<Vec<Technology>>())
}

pub fn set_auth_user(discord_id: String) -> Result<(), Error> {
    DB.get()
        .unwrap()
        .collection("authorized_users")
        .insert_one(AuthorizedUser { discord_id })
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

    Ok(())
}

pub fn is_auth_user(discord_id: String) -> Result<bool, Error> {
    Ok(DB
        .get()
        .unwrap()
        .collection::<AuthorizedUser>("authorized_users")
        .find_one(doc! { "discord_id": discord_id })
        .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
        .is_some())
}
