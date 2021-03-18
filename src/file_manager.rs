use crate::commons::util::fuzzy_id;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub const SESSION_ASSET_DIR: &str = "/Users/pmpower/assets/sessions";
pub const PROGRAM_ASSET_DIR: &str = "/Users/pmpower/assets/programs";
pub const USER_ASSET_DIR: &str = "/Users/pmpower/assets/users";
pub const PLATFORM_ASSET_DIR: &str = "/Users/pmpower/assets/platform";

pub async fn manage_notes_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
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
        let filepath = format!("{}/{}/notes/{}/{}", SESSION_ASSET_DIR, session_user_fuzzy_id, file_key, sanitize_filename::sanitize(&filename));
        file_paths.push(filepath.to_owned());

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    let json_response = serde_json::to_string(&file_paths)?;

    Ok(HttpResponse::Ok().content_type("application/json").body(json_response))
}

pub async fn manage_program_content(_request: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let program_fuzzy_id: String = _request.match_info().query("program_fuzzy_id").parse().unwrap();
    let purpose: String = _request.match_info().query("purpose").parse().unwrap();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();

        let filename = content_type.get_name().unwrap();

        // Ensure to create a directory for the program content.
        let dir_path = format!("{}/{}/{}", PROGRAM_ASSET_DIR, program_fuzzy_id, purpose);
        std::fs::create_dir_all(dir_path).unwrap();

        let file_path = format!("{}/{}/{}/{}", PROGRAM_ASSET_DIR, program_fuzzy_id, purpose, filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().body("Ok"))
}

pub async fn fetch_list_of_boards(_request: HttpRequest) -> Result<HttpResponse, Error> {
    let session_id: PathBuf = _request.match_info().query("session_id").parse().unwrap();

    let mut dir_name: PathBuf = PathBuf::from(SESSION_ASSET_DIR);
    dir_name.push(session_id);
    dir_name.push("boards");

    let mut entries = fs::read_dir(dir_name)?
        .map(|res| res.map(|e| e.file_name().into_string()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    entries.sort();

    let json_response = serde_json::to_string(&entries)?;

    Ok(HttpResponse::Ok().content_type("application/json").body(json_response))
}

/**
 * Especially for obtaining the bord files
 */
pub fn get_file_names(dir_name: PathBuf) -> Result<Vec<String>,std::io::Error> {
    let mut file_names: Vec<String> = Vec::new();

    let result = fs::read_dir(dir_name)?;
    for item in result {
        let dir_entry: fs::DirEntry = item?;
        if dir_entry.file_type()?.is_dir() {
            continue;
        }
        let entry = dir_entry.path();
        let name_result = entry.file_name();
        if name_result.is_some() {
            let file_name = format!("{:?}", name_result.unwrap());
            file_names.push(file_name);
        }
    }

    Ok(file_names)
}

pub async fn fetch_board_file(_request: HttpRequest) -> Result<NamedFile, Error> {
    let session_id: PathBuf = _request.match_info().query("session_id").parse().unwrap();
    let asset_name: PathBuf = _request.match_info().query("filename").parse().unwrap();

    let mut file_name: PathBuf = PathBuf::from(SESSION_ASSET_DIR);
    file_name.push(session_id);
    file_name.push("boards");
    file_name.push(asset_name);

    Ok(NamedFile::open(file_name)?)
}

pub async fn fetch_program_content(_request: HttpRequest) -> Result<NamedFile, Error> {
    let program_fuzzy_id: PathBuf = _request.match_info().query("program_fuzzy_id").parse().unwrap();
    let purpose: PathBuf = _request.match_info().query("purpose").parse().unwrap();
    let asset_name: PathBuf = _request.match_info().query("filename").parse().unwrap();

    let mut file_name: PathBuf = PathBuf::from(PROGRAM_ASSET_DIR);
    file_name.push(program_fuzzy_id);
    file_name.push(purpose);
    file_name.push(asset_name);

    Ok(NamedFile::open(file_name)?)
}

pub async fn fetch_platform_content(_request: HttpRequest) -> Result<NamedFile, Error> {
    let asset_name: PathBuf = _request.match_info().query("filename").parse().unwrap();

    let mut file_name: PathBuf = PathBuf::from(PLATFORM_ASSET_DIR);
    file_name.push(asset_name);

    Ok(NamedFile::open(file_name)?)
}

pub async fn manage_user_content(_request: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let user_id: String = _request.match_info().query("user_id").parse().unwrap();
 
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();

        let filename = content_type.get_name().unwrap();

        // Ensure to create a directory for the program content.
        let dir_path = format!("{}/{}", USER_ASSET_DIR, user_id);
        std::fs::create_dir_all(dir_path).unwrap();

        let file_path = format!("{}/{}/{}", USER_ASSET_DIR, user_id, filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().body("Ok"))
}

pub async fn fetch_user_content(_request: HttpRequest) -> Result<NamedFile, Error> {
    let user_id: PathBuf = _request.match_info().query("user_id").parse().unwrap();
    let asset_name: PathBuf = _request.match_info().query("filename").parse().unwrap();

    let mut file_name: PathBuf = PathBuf::from(USER_ASSET_DIR);
    file_name.push(user_id);
    file_name.push(asset_name);

    Ok(NamedFile::open(file_name)?)
}
