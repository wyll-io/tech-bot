use std::io::{Error, ErrorKind};

use juniper::{
    graphql_object, graphql_value, Context, EmptySubscription, FieldError, FieldResult, RootNode,
};
use polodb_core::{
    bson::{doc, Regex},
    Database,
};
use uuid::Uuid;

use crate::schema::{database, query};

pub struct DB {
    db: Database,
}

impl Context for DB {}

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<DB>>;

#[graphql_object(Context = DB)]
impl QueryRoot {
    fn technology(
        context: &DB,
        name: String,
        options: Option<String>,
        tags: Option<Vec<String>>,
    ) -> FieldResult<Vec<query::Technology>> {
        context
            .search_tech(
                name,
                options.unwrap_or(String::new()),
                &tags
                    .unwrap_or(Vec::new())
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
            )
            .map_err(map_fielderror)
            .map(|v| {
                v.iter()
                    .map(|v1| query::Technology {
                        _id: v1._id,
                        link: v1.link.clone().into(),
                        name: v1.name.clone().into(),
                        tags: v1.tags.clone().into(),
                        user_id: v1.user_id.to_string(),
                        created_at: v1.created_at.clone(),
                        updated_at: v1.updated_at.clone(),
                    })
                    .collect()
            })
    }

    fn technologies(context: &DB) -> FieldResult<Vec<query::Technology>> {
        context.list_tech().map_err(map_fielderror).map(|v| {
            v.iter()
                .map(|v1| query::Technology {
                    _id: v1._id,
                    link: v1.link.clone().into(),
                    name: v1.name.clone().into(),
                    tags: v1.tags.clone().into(),
                    user_id: v1.user_id.to_string(),
                    created_at: v1.created_at.clone(),
                    updated_at: v1.updated_at.clone(),
                })
                .collect()
        })
    }
}

#[graphql_object(Context = DB)]
impl MutationRoot {
    fn create_technology(
        context: &DB,
        name: String,
        link: String,
        tags: Vec<String>,
        user_id: String,
    ) -> FieldResult<query::Technology> {
        Ok(context
            .add_tech(
                &name,
                &link,
                &tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                user_id.parse::<i64>().map_err(|e| {
                    let err_str = e.to_string();
                    FieldError::new(
                        "failed to convert string user_id to i64",
                        graphql_value!({ "error": err_str }),
                    )
                })?,
            )
            .map_err(map_fielderror)?)
        .map(|v| v.into())
    }

    fn delete_technology(context: &DB, id: Uuid) -> FieldResult<Uuid> {
        context.remove_tech(id).map_err(map_fielderror)?;
        Ok(id)
    }

    fn delete_technologies(context: &DB, ids: Vec<Uuid>) -> FieldResult<Vec<Uuid>> {
        context
            .bulk_remove_technologies(ids.clone())
            .map_err(map_fielderror)?;
        Ok(ids)
    }

    fn update_technology(
        context: &DB,
        id: Uuid,
        name: Option<String>,
        link: Option<String>,
        tags: Option<Vec<String>>,
    ) -> FieldResult<bool> {
        context
            .edit_tech(id, name, link, tags)
            .map_err(map_fielderror)?;

        Ok(true)
    }
}

impl DB {
    pub fn new() -> Self {
        Self {
            db: Database::open_file(std::env::var("DB_PATH").expect("missing DB_PATH"))
                .expect("failed to initialize database"),
        }
    }

    /// Add a new technology to the database.
    pub fn add_tech(
        &self,
        name: &str,
        link: &str,
        tags: &[&str],
        user_id: i64,
    ) -> Result<database::Technology, Error> {
        let tech = database::Technology {
            _id: Uuid::new_v4(),
            link: link.into(),
            name: name.into(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            user_id,
            created_at: chrono::Utc::now().timestamp().to_string(),
            updated_at: None,
        };

        self.db
            .collection::<database::Technology>("technologies")
            .insert_one(&tech)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(tech)
    }

    pub fn remove_tech(&self, id: Uuid) -> Result<(), Error> {
        self.db
            .collection::<database::Technology>("technologies")
            .delete_one(doc! { "_id": id.to_string() })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }

    pub fn bulk_remove_technologies(&self, ids: Vec<Uuid>) -> Result<(), Error> {
        let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>();
        self.db
            .collection::<database::Technology>("technologies")
            .delete_many(doc! {
                "_id": {
                    "$in": ids
                }
            })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }

    pub fn list_tech(&self) -> Result<Vec<database::Technology>, Error> {
        Ok(self
            .db
            .collection("technologies")
            .find(None)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .filter(|doc| doc.is_ok())
            .map(|doc| doc.unwrap())
            .collect())
    }

    pub fn search_tech(
        &self,
        name: String,
        options: String,
        tags: &[&str],
    ) -> Result<Vec<database::Technology>, Error> {
        let mut query = doc! {};
        if !name.is_empty() {
            query.insert(
                "name",
                doc! {
                    "$regex": Regex {
                        pattern: name,
                        options: options,
                    }
                },
            );
        }
        if !tags.is_empty() {
            query.insert(
                "tags",
                doc! {
                    "$in": tags
                },
            );
        }
        Ok(self
            .db
            .collection::<database::Technology>("technologies")
            .find(query)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .map(
                |doc| doc.unwrap(), // todo: find a way to handle error
            )
            .collect::<Vec<database::Technology>>())
    }

    pub fn edit_tech(
        &self,
        id: Uuid,
        name: Option<String>,
        link: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<(), Error> {
        let mut update_doc = doc! {};
        update_doc.insert("updated_at", chrono::Utc::now().timestamp().to_string());

        if let Some(name) = name {
            update_doc.insert("name", name);
        }
        if let Some(link) = link {
            update_doc.insert("link", link);
        }
        if let Some(tags) = tags {
            update_doc.insert("tags", tags);
        }

        self.db
            .collection::<database::Technology>("technologies")
            .update_one(
                doc! {
                    "_id": id.to_string()
                },
                doc! {
                    "$set": update_doc
                },
            )
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }
}

fn map_fielderror(err: Error) -> FieldError {
    let err_str = err.to_string();
    match err.kind() {
        ErrorKind::NotFound => FieldError::new("not found", graphql_value!({ "error": err_str })),
        ErrorKind::PermissionDenied => {
            FieldError::new("permission denied", graphql_value!({ "error": err_str }))
        }
        ErrorKind::InvalidData => {
            FieldError::new("invalid data", graphql_value!({ "error": err_str }))
        }
        ErrorKind::Unsupported => FieldError::new(
            "unsupported operation",
            graphql_value!({ "error": err_str }),
        ),
        ErrorKind::Other => FieldError::new(
            "failed to search technology",
            graphql_value!({ "error": err_str }),
        ),
        _ => FieldError::new("internal error", graphql_value!({ "error": err_str })),
    }
}
