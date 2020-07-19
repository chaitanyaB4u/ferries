use juniper::{FieldResult, RootNode};


use crate::db_manager::MySqlConnectionPool;

use crate::models::users::{Registration, User};
use crate::models::teams::{NewTeamRequest,Team,TeamQuery};
use crate::models::sessions::{NewSessionRequest, ChangeSessionStateRequest, Session};
use crate::models::notes::{NewNoteRequest, Note};
use crate::models::programs::{NewProgramRequest, Program};
use crate::models::enrollments::{NewEnrollmentRequest, Enrollment,EnrollmentCriteria};
use crate::models::user_events::{get_events,EventRow,EventCriteria};
use crate::models::user_programs::{get_active_programs,ProgramRow,ProgramCriteria};


use crate::services::users::{get_users, register};
use crate::services::teams::{create_team,get_members};
use crate::services::sessions::{create_session,change_session_state};
use crate::services::notes::{create_new_note};
use crate::services::programs::{create_new_program};
use crate::services::enrollments::{create_new_enrollment,get_active_enrollments};

use crate::commons::chassis::{MutationResult,service_error,mutation_error};

#[derive(Clone)]
pub struct DBContext {
    pub db: MySqlConnectionPool,
}

impl juniper::Context for DBContext {}

pub struct QueryRoot;

#[juniper::object(Context = DBContext,description="Graph Query Root")]
impl QueryRoot {
    #[graphql(description = "Get the top 100 Users")]
    fn get_users(context: &DBContext) -> Vec<User> {
        let connection = context.db.get().unwrap();
        get_users(&connection)
    }

    #[graphql(description = "Get the Members of a Team")]
    fn get_members(context:&DBContext, team_query:TeamQuery) -> Vec<User> {
        let connection = context.db.get().unwrap();
        get_members(&connection,&team_query).expect("Something is Wrong")
    }

    #[graphql(description = "Get the Programs of a User")]
     fn get_programs(context:&DBContext, criteria:ProgramCriteria) -> Vec<ProgramRow> {
         let connection = context.db.get().unwrap();
         get_active_programs(&connection,criteria)
    }

    #[graphql(description = "Get the list of members enrolled into a Program")]
     fn get_enrollments(context:&DBContext, criteria:EnrollmentCriteria) -> Vec<User> {
         let connection = context.db.get().unwrap();
         get_active_enrollments(&connection,criteria).unwrap()
    }


    #[graphql(description = "Get the Events for a User")]
    fn get_sessions(context:&DBContext, criteria:EventCriteria) -> Vec<EventRow> { 
        let connection = context.db.get().unwrap();
        get_events(&connection,criteria)
    }
}


pub struct MutationRoot;

#[juniper::object(Context = DBContext)]
impl MutationRoot {
    
    fn create_user(context: &DBContext, registration: Registration) -> FieldResult<User> {
        let connection = context.db.get().unwrap();
        let reg_result = register(&connection, &registration);

        match reg_result {
            Ok(user) => Ok(user),
            Err(e) => Err(e)?,
        }
    }

    fn create_team(context: &DBContext, new_team_request: NewTeamRequest) -> FieldResult<Team> {
        let connection = context.db.get().unwrap();
        let result = create_team(&connection, &new_team_request);
        
        Ok(result.expect("Unable to Complete Creating the new team."))
    }

    fn create_program(context: &DBContext, new_program_request: NewProgramRequest) ->MutationResult<Program> {
       
        let errors = new_program_request.validate();
         if !errors.is_empty() {
             return MutationResult(Err(errors));
         }
 
         let connection = context.db.get().unwrap();
         let result = create_new_program(&connection, &new_program_request);
 
         match result {
             Ok(program) => MutationResult(Ok(program)),
             Err(e) => service_error(e)
         }
    }

    fn create_enrollment(context: &DBContext, new_enrollment_request: NewEnrollmentRequest) ->MutationResult<Enrollment> {
       
        let errors = new_enrollment_request.validate();
         if !errors.is_empty() {
             return MutationResult(Err(errors));
         }
 
         let connection = context.db.get().unwrap();
         let result = create_new_enrollment(&connection, &new_enrollment_request);
 
         match result {
             Ok(enrollment) => MutationResult(Ok(enrollment)),
             Err(e) => service_error(e)
         }
    }

    fn create_session(context: &DBContext, new_session_request: NewSessionRequest) -> MutationResult<Session> {

        let errors = new_session_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_session(&connection, &new_session_request);

        match result {
            Ok(session) => MutationResult(Ok(session)),
            Err(e) => service_error(e),
        }
    }

    fn alter_session_state(context: &DBContext, request: ChangeSessionStateRequest) -> MutationResult<String> {
        let connection = context.db.get().unwrap();
        let result = change_session_state(&connection, &request);
        
        match result {
            Ok(rows) => MutationResult(Ok(String::from("Ok"))),
            Err(e) => service_error(e),
        }
    }

    fn create_note(context: &DBContext, new_note_request: NewNoteRequest) -> MutationResult<Note> {

        let errors = new_note_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_new_note(&connection, &new_note_request);

        match result {
            Ok(note) => MutationResult(Ok(note)),
            Err(e) => mutation_error(e),
        }
    }
}

pub type GQSchema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_gq_schema() -> GQSchema {
    GQSchema::new(QueryRoot {}, MutationRoot {})
}
