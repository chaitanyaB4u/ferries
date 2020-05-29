// A convenience to group users. 
// People assemble here for a Purpose., eg buying a Jewellry

use crate::schema::teams;
use chrono::NaiveDateTime;
use uuid::Uuid;


#[derive(Queryable,Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub fuzzy_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Fields that we can safely expose to APIs
#[juniper::object(description = "Fields that we can safely expose to APIs")]
impl Team {
    pub fn fuzzy_id(&self) -> &str {
        self.fuzzy_id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// New Team Request is to create a New Team
// The Owner email shall be used in the future to add more members
#[derive(juniper::GraphQLInputObject)]
pub struct NewTeamRequest {
    pub owner_email: String,
    pub team_name: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct TeamQuery {
    pub fuzzy_id: String,
}


// The Persistable entity
#[derive(Insertable)]
#[table_name = "teams"]
pub struct NewTeam {
    pub name: String,
    pub fuzzy_id: String,
}

impl NewTeam  {
    pub fn from(new_team_request: &NewTeamRequest) -> NewTeam {
        let uuid = Uuid::new_v4();
        let hype = uuid.to_hyphenated().to_string();

        NewTeam {
                name:new_team_request.team_name.to_owned(),
                fuzzy_id: hype
        }
    }
}



