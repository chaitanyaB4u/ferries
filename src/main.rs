extern crate juniper;

#[macro_use]
extern crate diesel;

use std::sync::Arc;

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod commons;
mod db_manager;
mod file_manager;
mod graphql_schema;
mod models;
mod schema;
mod services;

use actix_files::NamedFile;
use db_manager::establish_connection;
use file_manager::{
    fetch_board_file, fetch_list_of_boards, fetch_program_content, fetch_user_content, manage_notes_file, manage_program_content, manage_user_content, PROGRAM_ASSET_DIR, SESSION_ASSET_DIR,
    USER_ASSET_DIR,
};
use graphql_schema::{create_gq_schema, DBContext, GQSchema};

use crate::services::discussions::get_pending_feed_count;

async fn upload_notes_file(payload: Multipart) -> Result<HttpResponse, Error> {
    manage_notes_file(payload).await
}

async fn upload_program_content(_request: HttpRequest, payload: Multipart) -> Result<HttpResponse, Error> {
    manage_program_content(_request, payload).await
}

async fn list_of_boards(_request: HttpRequest) -> Result<HttpResponse, Error> {
    fetch_list_of_boards(_request).await
}
async fn offer_board_file(_request: HttpRequest) -> Result<NamedFile, Error> {
    fetch_board_file(_request).await
}

async fn offer_program_content(_request: HttpRequest) -> Result<NamedFile, Error> {
    fetch_program_content(_request).await
}

async fn offer_user_content(_request: HttpRequest) -> Result<NamedFile, Error> {
    fetch_user_content(_request).await
}

async fn upload_user_content(_request: HttpRequest, payload: Multipart) -> Result<HttpResponse, Error> {
    manage_user_content(_request, payload).await
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8088/graphql");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

#[warn(unused_variables)]
async fn index(_request: HttpRequest) -> HttpResponse {
    let body = "Welcome to Ferris - 0.3 Version. The API for the Coaching Assistant.";
    HttpResponse::Ok().body(body)
}

async fn graphql(ctx: web::Data<DBContext>, schema: web::Data<Arc<GQSchema>>, request: web::Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
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

    Ok(HttpResponse::Ok().content_type("application/json").body(&result))
}

async fn count_feeds(_request: HttpRequest, ctx: web::Data<DBContext>) -> Result<HttpResponse, Error> {
    let connection = ctx.db.get().unwrap();

    let user_id = _request.match_info().query("user_id");
    let result = get_pending_feed_count(&connection, user_id);
    let json_response = serde_json::to_string(&result)?;

    Ok(HttpResponse::Ok().content_type("application/json").body(json_response))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    std::fs::create_dir_all(SESSION_ASSET_DIR).unwrap();
    std::fs::create_dir_all(PROGRAM_ASSET_DIR).unwrap();
    std::fs::create_dir_all(USER_ASSET_DIR).unwrap();

    let pool = establish_connection();
    let db_context = DBContext { db: pool.clone() };

    let gq_schema = std::sync::Arc::new(create_gq_schema());
    let bind = dotenv::var("BIND").unwrap();
    println!("Server is running at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(db_context.clone())
            .data(gq_schema.clone())
            .wrap(Cors::new().supports_credentials().max_age(3600).finish())
            .route("graphql", web::post().to(graphql))
            .route("graphiql", web::get().to(graphiql))
            .route("assets/upload", web::post().to(upload_notes_file))
            .route("assets/boards/{session_user_fuzzy_id}", web::get().to(list_of_boards))
            .route("assets/boards/{session_user_fuzzy_id}/{filename}", web::get().to(offer_board_file))
            .route("assets/users/{user_id}", web::post().to(upload_user_content))
            .route("assets/users/{user_id}/{filename}", web::get().to(offer_user_content))
            .route("assets/programs/{program_fuzzy_id}/{purpose}", web::post().to(upload_program_content))
            .route("assets/programs/{program_fuzzy_id}/{purpose}/{filename}", web::get().to(offer_program_content))
            .route("feeds/{user_id}", web::get().to(count_feeds))
            .route("/", web::get().to(index))
    })
    .bind(&bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_graphql() {}
}
