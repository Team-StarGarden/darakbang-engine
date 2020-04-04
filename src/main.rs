#[macro_use]
extern crate diesel;

use std::io;
use std::sync::Arc;

use actix_rt;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{dev, http, middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::http::{playground::playground_source, GraphQLRequest};
use middleware::errhandlers::ErrorHandlerResponse;

use crate::core::Server;
use crate::gql::{Context, Mutation, Query, Schema};

mod config;
mod core;
mod database;
mod gql;
mod log;
mod protocol;
mod websocket;

/// A playground for developers
async fn playground() -> impl Responder {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// GraphQL handler
async fn graphql(
    schema: web::Data<Arc<Schema>>,
    context: web::Data<Arc<Context>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

fn render_404<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    dbg!(res.request());
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let configuration = config::Config::load().expect("Invalid configuration detected");

    let pool = database::establish_connection(&configuration.database)
        .expect("Invalid database configuration detected");

    log::setup_logger().expect("Logger setup failed");

    let context = Arc::new(Context::new(pool));
    let schema = Arc::new(Schema::new(Query, Mutation));

    let server = Server::default();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(context.clone())
            .data(server.clone())
            .wrap(middleware::Logger::default())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, render_404))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphql").route(web::get().to(graphql)))
            .service(web::resource("/").route(web::get().to(playground)))
            .service(web::resource("/ws").to(websocket::ws))
    })
    .bind(&configuration.bind_address)?
    .run()
    .await
}
