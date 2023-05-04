use polodb_core::{
    bson::{doc, oid::ObjectId},
    Database as DB,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Technology {
    link: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizedUser {
    name: String,
}

pub struct Database {
    pub db: DB,
}

impl Database {
    /// Create a new database instance.
    pub fn new() -> Self {
        Self {
            db: DB::open_file("test-polo.db").expect("failed to initialize database"),
        }
    }

    /// Add a new technology to the database.
    pub fn add_tech(&self, link: String, name: String) -> Result<String, Error> {
        Ok(self
            .db
            .collection::<Technology>("technologies")
            .insert_one(Technology { link, name })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .inserted_id
            .to_string())
    }

    pub fn remove_tech(&self, id: &str) -> Result<(), Error> {
        self.db
            .collection::<Technology>("technologies")
            .delete_one(doc! { "_id": Some(ObjectId::from_str(id).map_err(|err| Error::new(ErrorKind::InvalidInput, err))?) })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }

    pub fn list_tech(&self) -> Result<Vec<Technology>, Error> {
        Ok(self
            .db
            .collection("technologies")
            .find(None)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .filter(|doc| doc.is_ok())
            .map(|doc| doc.unwrap())
            .collect())
    }

    pub fn search_tech(&self, name: String) -> Result<Technology, Error> {
        if let Some(tech) = self
            .db
            .collection::<Technology>("technologies")
            .find(doc! { "$eq": [{"name": name}] })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .next()
        {
            Ok(tech.map_err(|err| Error::new(ErrorKind::InvalidInput, err))?)
        } else {
            Err(Error::new(ErrorKind::NotFound, "Technology not found"))
        }
    }
}
