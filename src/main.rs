
extern crate juniper;

#[macro_use]
extern crate diesel;

use std::sync::Arc;

use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod db_manager;
mod graphql_schema;
mod models;
mod schema;
mod services;

use db_manager::establish_connection;
use graphql_schema::{create_gq_schema, DBContext, GQSchema};

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8088/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    schema: web::Data<Arc<GQSchema>>,
    ctx: web::Data<DBContext>,
    request: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let res = request.execute(&schema, &ctx);
        let json_response = serde_json::to_string(&res)?;

        Ok::<_, serde_json::error::Error>(json_response)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(&result))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = establish_connection();
    let db_context = DBContext { db: pool.clone() };

    let gq_schema = std::sync::Arc::new(create_gq_schema());
    let bind = "127.0.0.1:8088";
    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(db_context.clone())
            .data(gq_schema.clone())
            .service(graphql)
            .service(graphiql)
    })
    .bind(&bind)?
    .run()
    .await
}
