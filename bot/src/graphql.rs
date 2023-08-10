use graphql_client::GraphQLQuery;
use uuid::Uuid;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct CreateTechnology;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct GetTechnologies;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct GetTechnology;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct DeleteTechnology;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct DeleteTechnologies;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct UpdateTechnology;
