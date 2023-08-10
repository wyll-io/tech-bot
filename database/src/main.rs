use std::{
    io,
    sync::{Arc, OnceLock},
};

use actix_cors::Cors;
use actix_web::{
    get,
    middleware::{self, Logger},
    route,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptySubscription,
};

use crate::graphql::{MutationRoot, QueryRoot, Schema, DB};

mod graphql;
mod schema;

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql_handler_route(st: Data<Schema>, data: Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, CONTEXT.get().unwrap()).await;
    HttpResponse::Ok().json(user)
}

static CONTEXT: OnceLock<DB> = OnceLock::new();

#[actix_web::main]
async fn main() -> io::Result<()> {
    match dotenvy::from_path(std::env::var("DOTENV_PATH").unwrap_or_else(|_| "/.env".to_string())) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to load .env file: {}", e);
        }
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::new(),
    ));
    CONTEXT.get_or_init(|| DB::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::from(schema.clone()))
            .service(graphql_handler_route)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind((
        std::env::var("HOST").unwrap_or(String::from("127.0.0.1")),
        std::env::var("PORT")
            .unwrap_or(String::from("8080"))
            .parse::<u16>()
            .expect("failed to convert port to u16"),
    ))?
    .run()
    .await
}
