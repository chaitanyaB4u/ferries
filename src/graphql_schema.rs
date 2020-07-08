use juniper::{FieldResult, RootNode};


use crate::db_manager::MySqlConnectionPool;

use crate::models::users::{Registration, User};
use crate::models::teams::{NewTeamRequest,Team,TeamQuery};
use crate::models::sessions::{NewSessionRequest, Session};

use crate::services::users::{get_users, register};
use crate::services::teams::{create_team,get_members};
use crate::services::sessions::{create_session};

use crate::commons::chassis::{MutationResult,session_service_error};

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


    fn create_session(context: &DBContext, new_session_request: NewSessionRequest) -> MutationResult<Session> {
        let connection = context.db.get().unwrap();

        let errors = new_session_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

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
