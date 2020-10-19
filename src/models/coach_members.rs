use diesel::prelude::*;

use crate::models::enrollments::{Enrollment,EnrollmentFilter};
use crate::models::programs::Program;
use crate::models::users::User;

use crate::schema::enrollments::dsl::*;
use crate::schema::programs::dsl::*;
use crate::schema::users::dsl::*;

#[derive(juniper::GraphQLInputObject)]
pub struct CoachCriteria {
    pub coach_id: String,
    pub program_id: Option<String>,
    pub desire: EnrollmentFilter,
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
    let mut query = enrollments
        .inner_join(users)
        .inner_join(programs)
        .filter(coach_id.eq(criteria.coach_id))
        .order_by(full_name.asc())
        .into_boxed();

    if let EnrollmentFilter::NEW = criteria.desire {
        query = query.filter(is_new.eq(true));
    }

    if criteria.program_id.is_some() {
        query = query.filter(program_id.eq(criteria.program_id.unwrap()));
    }

    let result: Vec<EnrollmentType> = query.load(connection)?;

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
