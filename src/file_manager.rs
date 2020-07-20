use crate::commons::util::fuzzy_id;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;

pub const ASSET_DIR: &'static str = "/Users/harinimaniam/assets";

pub async fn manage_file_assets(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut file_paths: Vec<String> = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let session_user_fuzzy_id = content_type.get_name().unwrap();
        let file_key = fuzzy_id();

        // Ensure to create a directory for the session_user.
        let dir_path = format!("{}/{}/notes/{}", ASSET_DIR, session_user_fuzzy_id, file_key);
        std::fs::create_dir_all(dir_path).unwrap();

        // Now we
        let filepath = format!(
            "{}/{}/notes/{}/{}",
            ASSET_DIR,
            session_user_fuzzy_id,
            file_key,
            sanitize_filename::sanitize(&filename)
        );
        file_paths.push(filepath.to_owned());

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

    let json_response = serde_json::to_string(&file_paths)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response))
}

pub async fn fetch_board_file(_request: HttpRequest) -> Result<HttpResponse, Error> {

    let file_path = "/Users/harinimaniam/assets/d91e5527-9cc3-4d56-9c69-d386c9cba535/board/Board_1";

    let content = fs::read_to_string(file_path)?;

    Ok(HttpResponse::Ok().body(content))
}
