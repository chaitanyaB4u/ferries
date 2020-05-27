use juniper::{FieldResult, RootNode};

use crate::db_manager::MySqlConnectionPool;

use crate::models::users::{Registration, User};
use crate::models::teams::{NewTeamRequest,Team};

use crate::services::users::{get_users, register};
use crate::services::teams::{create_team};

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
}

pub struct MutationRoot;

#[juniper::object(Context = DBContext)]
impl MutationRoot {
    fn create_user(context: &DBContext, registration: Registration) -> FieldResult<User> {
        let connection = context.db.get().unwrap();
        let reg_result = register(&connection, &registration);
        Ok(reg_result.expect("Unable to Complete Registration."))
    }

    fn create_team(context: &DBContext, new_team_request: NewTeamRequest) -> FieldResult<Team> {
        let connection = context.db.get().unwrap();
        let result = create_team(&connection, &new_team_request);
        Ok(result.expect("Unable to Complete Creating the new team."))
    }
}

pub type GQSchema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_gq_schema() -> GQSchema {
    GQSchema::new(QueryRoot {}, MutationRoot {})
}
