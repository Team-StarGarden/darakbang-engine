// for the rocket
#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
extern crate dotenv;

use rocket::{get, post, routes};
use rocket::response::content::Html;
use rocket::State;

use crate::gql::{Context, Mutation, Query, Schema};

mod gql;
mod database;

/// A playground for developers
#[get("/")]
fn playground() -> Html<String> {
    juniper_rocket::playground_source("/graphql")
}

/// GraphQL handler for GET requests
#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

/// GraphQL handler for POST requests
#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    database::establish_connection();
    rocket::ignite()
        .manage(Context::new())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![playground, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
