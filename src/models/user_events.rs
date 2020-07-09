use crate::models::programs::Program;
use crate::models::sessions::Session;
use diesel::prelude::*;

#[derive(juniper::GraphQLInputObject)]
pub struct EventCriteria {
    user_fuzzy_id: String,
}

#[derive(juniper::GraphQLObject)]
pub struct EventRow {
    session_id: i32,
    session_name: String,
    session_description: Option<String>,
    program_id: i32,
    program_name: String,
    coach_id: i32,
}

impl EventRow {
    pub fn new(item: (Session, Program)) -> EventRow {
        let session: Session = item.0;
        let program: Program = item.1;

        EventRow {
            session_id: session.id,
            session_name: session.name,
            session_description: session.description,
            program_id: program.id,
            program_name: program.name,
            coach_id: program.coach_id,
        }
    }
}

pub fn get_events(connection: &MysqlConnection, criteria: EventCriteria) -> Vec<EventRow> {
    use crate::schema::programs::dsl::*;
    use crate::schema::sessions::dsl::*;

    let data = sessions.inner_join(programs).load(connection);

    let rows: Vec<(Session, Program)> = data.unwrap();

    let mut event_rows: Vec<EventRow> = Vec::new();

    for row in rows {
        event_rows.push(EventRow::new(row));
    }

    event_rows
}
