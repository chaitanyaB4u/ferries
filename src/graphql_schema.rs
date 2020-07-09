use juniper::{FieldResult, RootNode};


use crate::db_manager::MySqlConnectionPool;

use crate::models::users::{Registration, User};
use crate::models::teams::{NewTeamRequest,Team,TeamQuery};
use crate::models::sessions::{NewSessionRequest, Session};
use crate::models::programs::{NewProgramRequest, Program};
use crate::models::user_events::{get_events,EventRow,EventCriteria};

use crate::services::users::{get_users, register};
use crate::services::teams::{create_team,get_members};
use crate::services::sessions::{create_session};
use crate::services::programs::{create_new_program};

use crate::commons::chassis::{MutationResult,session_service_error,program_service_error};

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
            Err(e) => program_service_error(e)
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
            Err(e) => session_service_error(e),
        }
    }
}

pub type GQSchema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_gq_schema() -> GQSchema {
    GQSchema::new(QueryRoot {}, MutationRoot {})
}
