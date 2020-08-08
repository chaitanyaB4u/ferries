use diesel::prelude::*;

use crate::models::notes::{NewNoteRequest,NewNote, NewNoteFile, Note};

use crate::services::sessions::find_session_user;

use crate::schema::session_notes::dsl::*;
use crate::schema::session_files::dsl::*;


pub fn create_new_note(connection: &MysqlConnection, request: &NewNoteRequest) -> QueryResult<Note> {

    let the_session_user_id = &request.session_user_id.as_str();

    let session_user = find_session_user(connection, the_session_user_id)?;

    let new_note = NewNote::from(request,session_user); 
    
    diesel::insert_into(session_notes).values(&new_note).execute(connection)?;
  
    let note: Note = find(connection, &new_note.id.as_str())?;

    insert_files(connection,request,&note)?;

    Ok(note)
}

fn insert_files(connection: &MysqlConnection, request: &NewNoteRequest, note: &Note) -> QueryResult<usize> {

    if request.files.is_none() {
         return Ok(0);
    }

    let insert_files: Vec<NewNoteFile> = request.files.as_ref().unwrap()
                .iter()
                .map(|file| NewNoteFile::from(file,note.id.to_owned()))
                .collect();

    diesel::insert_into(session_files).values(insert_files).execute(connection)
}

fn find(connection: &MysqlConnection,the_id: &str) -> QueryResult<Note> {

    use crate::schema::session_notes::dsl::id;

    session_notes.filter(id.eq(the_id)).first(connection)
}