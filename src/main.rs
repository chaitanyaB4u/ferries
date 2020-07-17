extern crate juniper;

#[macro_use]
extern crate diesel;

use std::sync::Arc;
use std::io::Write;

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use futures::{StreamExt, TryStreamExt};


mod commons;
mod db_manager;
mod graphql_schema;
mod models;
mod schema;
mod services;


use db_manager::establish_connection;
use graphql_schema::{create_gq_schema, DBContext, GQSchema};

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {

    while let Ok(Some(mut field)) = payload.try_next().await {
  
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let name = content_type.get_name().unwrap();

        println!("fuzzyId is {:?}",name);
        
        let dir_path = format!("./tmp/{}",name);
        std::fs::create_dir_all(dir_path).unwrap();

        let filepath = format!("./tmp/{}/{}", name,sanitize_filename::sanitize(&filename));
        
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8088/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[warn(unused_variables)]
async fn index(_request: HttpRequest) -> HttpResponse {
    let body = format!("Welcome to Ferris. The API for the Coaching Assistant!!!");
    HttpResponse::Ok().body(body)
}

async fn graphql(
    ctx: web::Data<DBContext>,
    schema: web::Data<Arc<GQSchema>>,
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
    std::fs::create_dir_all("./tmp").unwrap();
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = establish_connection();
    let db_context = DBContext { db: pool.clone() };

    let gq_schema = std::sync::Arc::new(create_gq_schema());
    let bind = "localhost:8088";
    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(db_context.clone())
            .data(gq_schema.clone())
            .wrap(Cors::new().supports_credentials().max_age(3600).finish())
            .route("graphql",web::post().to(graphql))
            .route("graphiql",web::get().to(graphiql))
            .route("upload",web::post().to(upload))
            .route("/",web::get().to(index))
    })
    .bind(&bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use actix_web::{test,http};
    use actix_rt;

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_graphql() {
        
    }
}

