use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Technology found in database")]
pub struct Technology {
    #[graphql(description = "Technology's unique ID")]
    pub _id: Uuid,
    #[graphql(description = "Link to technology")]
    pub link: String,
    #[graphql(description = "Name of technology")]
    pub name: String,
    #[graphql(description = "Technology tags (e.g. 'javascript', 'react')")]
    pub tags: Vec<String>,
    #[graphql(description = "user's Discord ID")]
    pub user_id: String,
    #[graphql(description = "Technology creation timestamp")]
    pub created_at: String,
    #[graphql(description = "Technology update timestamp")]
    pub updated_at: Option<String>,
}
