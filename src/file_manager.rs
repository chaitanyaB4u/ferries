use crate::commons::util::fuzzy_id;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use actix_files::NamedFile;
use std::path::PathBuf;


pub const SESSION_ASSET_DIR: &'static str = "/Users/harinimaniam/assets/sessions";

pub const PROGRAM_ASSET_DIR: &'static str = "/Users/harinimaniam/assets/programs";

pub async fn manage_file_assets(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut file_paths: Vec<String> = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let session_user_fuzzy_id = content_type.get_name().unwrap();
        let file_key = fuzzy_id();

        // Ensure to create a directory for the session_user.
        let dir_path = format!("{}/{}/notes/{}", SESSION_ASSET_DIR, session_user_fuzzy_id, file_key);
        std::fs::create_dir_all(dir_path).unwrap();

        // Now we
        let filepath = format!(
            "{}/{}/notes/{}/{}",
            SESSION_ASSET_DIR,
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

pub async fn fetch_board_file(_request: HttpRequest) -> Result<NamedFile,Error> {

    let session_user_fuzzy_id: PathBuf = _request.match_info().query("session_user_fuzzy_id").parse().unwrap();
    let asset_name: PathBuf = _request.match_info().query("filename").parse().unwrap();

    let mut file_name: PathBuf = PathBuf::from(SESSION_ASSET_DIR);
    file_name.push(session_user_fuzzy_id);
    file_name.push("boards");
    file_name.push(asset_name);

    Ok(NamedFile::open(file_name)?)
}

pub async fn fetch_program_cover_file(_request: HttpRequest) -> Result<NamedFile,Error> {

    let program_fuzzy_id: PathBuf = _request.match_info().query("program_fuzzy_id").parse().unwrap();
    let asset_name: PathBuf = _request.match_info().query("filename").parse().unwrap();

    let mut file_name: PathBuf = PathBuf::from(PROGRAM_ASSET_DIR);
    file_name.push(program_fuzzy_id);
    file_name.push("cover");
    file_name.push(asset_name);

    if !file_name.exists() {
        file_name = get_no_file_path();
    }

    Ok(NamedFile::open(file_name)?)
}

fn get_no_file_path() -> PathBuf {

    let mut file_name = PathBuf::from(PROGRAM_ASSET_DIR);
    file_name.push("cover.png");

    file_name
}