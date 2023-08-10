use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
#[graphql(description = "New technology to add to database")]
pub struct Technology {
    #[graphql(description = "Name of technology")]
    pub name: String,
    #[graphql(description = "Link to technology")]
    pub link: String,
    #[graphql(description = "Technology tags (e.g. 'javascript', 'react')")]
    pub tags: Vec<String>,
    #[graphql(description = "user's Discord ID")]
    pub user_id: f64,
}
