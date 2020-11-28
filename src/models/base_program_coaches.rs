use crate::commons::util;
use crate::schema::base_program_coaches;
use chrono::NaiveDateTime;

#[derive(Queryable,Debug)]
pub struct BaseProgramCoach {
    pub id: String,
    pub base_program_id: String,
    pub coach_id: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(description = "The fields we offer to outside world")]
impl BaseProgramCoach {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn base_program_id(&self) -> &str {
        self.base_program_id.as_str()
    }

    pub fn coach_id(&self) -> &str {
        self.coach_id.as_str()
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}

// The Persistable entity with the Fuzzy_id injected by us.
#[derive(Insertable)]
#[table_name = "base_program_coaches"]
pub struct NewBaseProgramCoach {
    pub id: String,
    pub base_program_id: String,
    pub coach_id: String,
    pub is_admin: bool,
}


#[derive(juniper::GraphQLInputObject)]
pub struct AssociateCoachRequest {
    pub base_program_id: String,
    pub coach_id: String,
    pub admin_coach_id: String,
    pub is_admin:bool,
}

impl NewBaseProgramCoach {
    pub fn from(request: &AssociateCoachRequest) -> NewBaseProgramCoach {
        let fuzzy_id = util::fuzzy_id();

        NewBaseProgramCoach {
            id: fuzzy_id,
            base_program_id: request.base_program_id.to_owned(),
            coach_id: request.coach_id.to_owned(),
            is_admin: request.is_admin,
        }
    }
}
