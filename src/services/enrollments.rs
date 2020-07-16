use diesel::prelude::*;

use crate::models::users::{User};
use crate::models::enrollments::{NewEnrollmentRequest,Enrollment,EnrollmentCriteria};

use crate::services::programs::find_by_fuzzy_id;

use crate::schema::enrollments::dsl::*;
use crate::schema::team_members::dsl::*;
use crate::schema::users::dsl::*;
use crate::schema::teams::dsl::*;

const ERROR_001: &'static str = "Error in finding any prior enrollment. Error-001.";
const WARNING: &'static str = "It seems you have already enrolled the team to this program";
const ERROR_002: &'static str = "Error in creating enrollment. Error-002.";
const ERROR_003: &'static str = "Error in finding any post-enrollment. Error-003.";
const QUERY_ERROR: &'static str = "Error in fetching enrolled members";


pub fn create_new_enrollment(connection: &MysqlConnection, request: &NewEnrollmentRequest) -> Result<Enrollment,&'static str> {

    let result: QueryResult<Enrollment> = find_by_request(connection,request);
    if result.is_err() {
        return Err(ERROR_001);
    }
    
    let prior_enrollment = result.unwrap();
    if prior_enrollment.id > 0 {
        return Err(WARNING);   
    }

    let insert_result = diesel::insert_into(enrollments)
        .values(request)
        .execute(connection);

    if insert_result.is_err() {
        return Err(ERROR_002);
    } 
    
    let finder_result: QueryResult<Enrollment> = find_by_request(connection,request);

    match finder_result {
        Ok(enrollment) => Ok(enrollment),
        Err(_) => {
            return Err(ERROR_003);
        }
    } 
    
}

fn find_by_request(connection: &MysqlConnection,request: &NewEnrollmentRequest) -> QueryResult<Enrollment> {
    
    use crate::schema::enrollments::dsl::team_id;

    enrollments
        .filter(program_id.eq(request.program_id))
        .filter(team_id.eq(request.team_id))
        .first(connection)
}

pub fn get_active_enrollments(connection: &MysqlConnection, criteria: EnrollmentCriteria) -> Result<Vec<User>,&'static str> {

    let program = find_by_fuzzy_id(connection,criteria.program_fuzzy_id.as_str())?;

    let result: QueryResult<Vec<User>>  = enrollments
        .inner_join(teams.inner_join(team_members.inner_join(users)))
        .filter(program_id.eq(program.id))
        .select(users::all_columns())
        .load(connection);
        
    if result.is_err() {
        return Err(QUERY_ERROR);
    }

    Ok(result.unwrap())
} 