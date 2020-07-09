use crate::models::programs::Program;
use crate::models::sessions::Session;
use diesel::prelude::*;
use crate::schema::programs::dsl::*;
use crate::schema::sessions::dsl::*;


#[derive(juniper::GraphQLInputObject)]
pub struct EventCriteria {
    user_fuzzy_id: String,
}

pub struct EventRow {
    pub session:Session,
    pub program:Program,
}

#[juniper::object]
impl EventRow {
   
    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

}


pub fn get_events(connection: &MysqlConnection, criteria: EventCriteria) -> Vec<EventRow> {
   
    let data = sessions.inner_join(programs).load(connection);

    let rows: Vec<(Session, Program)> = data.unwrap();

    let mut event_rows: Vec<EventRow> = Vec::new();

    for row in rows {
        event_rows.push(EventRow{session:row.0,program:row.1});
    }

    event_rows
}
