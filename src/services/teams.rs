use diesel::prelude::*;
use crate::models::teams::{NewTeam,NewTeamRequest,Team};
use crate::schema::teams::dsl::*;

fn find_by_fuzzy_id(connection: &MysqlConnection,fuzzy: &str) -> QueryResult<Team> {
    teams
        .filter(fuzzy_id.eq(fuzzy))
        .first(connection)
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