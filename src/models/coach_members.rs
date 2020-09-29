use diesel::prelude::*;

use crate::models::enrollments::Enrollment;
use crate::models::programs::Program;
use crate::models::users::User;

use crate::schema::enrollments::dsl::*;
use crate::schema::programs::dsl::*;
use crate::schema::users::dsl::*;

#[derive(juniper::GraphQLInputObject)]
pub struct CoachCriteria {
    pub coach_id: String,
}

pub struct MemberRow {
    pub enrollment: Enrollment,
    pub user: User,
    pub program: Program,
}

#[juniper::object]
impl MemberRow {
    pub fn enrollment(&self) -> &Enrollment {
        &self.enrollment
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

type EnrollmentType = (Enrollment, User, Program);

pub fn get_coach_members(connection: &MysqlConnection, criteria: CoachCriteria) -> Result<Vec<MemberRow>, diesel::result::Error> {
    let given_coach_id = criteria.coach_id;

    let result: Vec<EnrollmentType> = enrollments.inner_join(users).inner_join(programs).filter(coach_id.eq(&given_coach_id)).load(connection)?;

    let mut rows: Vec<MemberRow> = Vec::new();

    for item in result {

        let row = MemberRow {
            enrollment: item.0,
            user: item.1,
            program: item.2,
        };

        rows.push(row);
    }


    Ok(rows)
}
