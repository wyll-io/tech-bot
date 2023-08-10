use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::query;

#[derive(Serialize, Deserialize)]
pub struct Technology {
    pub _id: Uuid,
    pub link: String,
    pub name: String,
    pub tags: Vec<String>,
    pub user_id: i64,
    pub created_at: String, // can't use chrono::NaiveDateTime bc it fails to deserialize
    pub updated_at: Option<String>, // can't use chrono::NaiveDateTime bc it fails to deserialize
}

impl Into<query::Technology> for Technology {
    fn into(self) -> query::Technology {
        query::Technology {
            _id: self._id,
            link: self.link,
            name: self.name,
            tags: self.tags,
            user_id: self.user_id.to_string(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
