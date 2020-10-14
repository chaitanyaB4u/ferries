use diesel::prelude::*;

use crate::models::enrollments::Enrollment;
use crate::models::sessions::Session;
use crate::models::notes::Note;

use crate::schema::session_notes;

use crate::schema::enrollments::dsl::*;
use crate::schema::sessions::dsl::*;
use crate::schema::session_notes::dsl::*;

#[derive(juniper::GraphQLInputObject)]
pub struct ArtifactCriteria {
    pub program_id: String,
    pub user_id: String,
}

pub struct NoteRow {
    pub session: Session,
    pub note: Note,
}

#[juniper::object]
impl NoteRow {
    pub fn session(&self) -> &Session {
        &self.session
    }
    pub fn note(&self) -> &Note {
        &self.note
    }
}

pub fn get_program_notes(connection: &MysqlConnection, criteria: ArtifactCriteria) ->Result<Vec<NoteRow>,diesel::result::Error> {

    type Row = (Enrollment,(Session,Note));

    let artifact_rows: Vec<Row> = enrollments
        .inner_join(sessions.inner_join(session_notes))
        .filter(member_id.eq(&criteria.user_id))
        .filter(crate::schema::sessions::program_id.eq(&criteria.program_id))
        .order_by(session_notes::updated_at.asc())
        .load(connection)?;

    let mut rows: Vec<NoteRow> = Vec::new(); 
    
    for item in artifact_rows {
        let session = (item.1).0;
        let note = (item.1).1;

        let note_row = NoteRow{session,note};

        rows.push(note_row);
    }

    Ok(rows)
}
