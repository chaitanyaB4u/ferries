use diesel::prelude::*;
use crate::models::users::{User};
use crate::models::teams::{NewTeam,NewTeamRequest,Team,TeamQuery};
use crate::schema::users;
use crate::schema::teams;
use crate::schema::teams::dsl::*;
use crate::schema::team_members::dsl::*;

fn find_by_fuzzy_id(connection: &MysqlConnection,fuzzy: &str) -> QueryResult<Team> {
    teams
        .filter(teams::fuzzy_id.eq(fuzzy))
        .first(connection)
}

pub fn get_members(connection: &MysqlConnection,team_query: &TeamQuery) -> QueryResult<Vec<User>> {
    
    let members = team_members
        .inner_join(users::table)
        .inner_join(teams::table)
        .filter(teams::fuzzy_id.eq(team_query.fuzzy_id.to_owned()))
        .select(users::all_columns)
        .load(connection);

    members    
}

pub fn create_team(connection: &MysqlConnection, new_team_request: &NewTeamRequest) -> Result<Team,&'static str>{

    let new_team = NewTeam::from(new_team_request);

    diesel::insert_into(teams)
        .values(&new_team)
        .execute(connection)
        .expect("Error while creating a New Team");    
    
    let result = find_by_fuzzy_id(connection, new_team.fuzzy_id.as_str());
    
    match result {
        Ok(team) => Ok(team),
        Err(_) => {
            return Err("Unable to Create Team");
        }
    }
}