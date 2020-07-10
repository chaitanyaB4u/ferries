use diesel::prelude::*;

use crate::models::notes::{NewNoteRequest,NewNote, NewNoteFile, Note};
use crate::schema::session_notes::dsl::*;
use crate::schema::session_files::dsl::*;

pub fn create_new_note(connection: &MysqlConnection, request: &NewNoteRequest) -> Result<Note,&'static str> {

    let new_note = NewNote::from(request); 

    let result = diesel::insert_into(session_notes).values(&new_note).execute(connection);
    if result.is_err() {

    }

    let finder_result = find_note_by_fuzzy_id(connection, &new_note.fuzzy_id.as_str());    
    if finder_result.is_err() {

    }

    let note = finder_result.unwrap();

    let file_result = insert_files(connection,request,&note);
    if file_result.is_err() {
        
    }
    Ok(note)
}

fn insert_files(connection: &MysqlConnection, request: &NewNoteRequest, note: &Note) -> Result<usize, diesel::result::Error> {

    if request.files.is_none() {
         return Ok(0);
    }

    let insert_files: Vec<NewNoteFile> = request.files.as_ref().unwrap()
                .iter()
                .map(|file| NewNoteFile::from(file,note.id))
                .collect();

    diesel::insert_into(session_files).values(insert_files).execute(connection)
}

fn find_note_by_fuzzy_id(connection: &MysqlConnection,fuzzy: &str) -> QueryResult<Note> {

    use crate::schema::session_notes::dsl::fuzzy_id;

    session_notes.filter(fuzzy_id.eq(fuzzy)).first(connection)
}