use diesel::prelude::*;

use crate::models::users::User;
use crate::models::programs::Program;

use crate::models::enrollments::{NewEnrollmentRequest,NewEnrollment, Enrollment,EnrollmentCriteria};

use crate::services::users;
use crate::services::programs;

use crate::schema::enrollments::dsl::*;


const ERROR_001: &'static str = "Error in finding any prior enrollment. Error-001.";
const WARNING: &'static str = "It seems you have already enrolled the team to this program";
const ERROR_002: &'static str = "Error in creating enrollment. Error-002.";
const ERROR_003: &'static str = "Error in finding any post-enrollment. Error-003.";
const QUERY_ERROR: &'static str = "Error in fetching enrolled members";

pub fn create_new_enrollment(connection: &MysqlConnection, request: &NewEnrollmentRequest) -> Result<Enrollment,&'static str> {

    let user: User = users::find_by_fuzzy_id(connection,request.user_fuzzy_id.as_str())?;
    let program: Program = programs::find_by_fuzzy_id(connection,request.program_fuzzy_id.as_str())?;

    gate_prior_enrollment(connection, &program, &user)?;
    insert_enrollment(connection, &program, &user)?;

    let finder_result: QueryResult<Enrollment> = find_enrollment(connection,&program,&user);

    match finder_result {
        Ok(enrollment) => Ok(enrollment),
        Err(_) => {
            return Err(ERROR_003);
        }
    } 
    
}

fn insert_enrollment (connection: &MysqlConnection,program: &Program, user: &User) -> Result<usize, &'static str> {
    let enrollment: NewEnrollment = NewEnrollment::from(&program,&user);
    let insert_result = diesel::insert_into(enrollments)
        .values(enrollment)
        .execute(connection);

    if insert_result.is_err() {
        return Err(ERROR_002);
    }
    
    Ok(insert_result.unwrap())
}

fn gate_prior_enrollment(connection: &MysqlConnection,program: &Program, user: &User) -> Result<bool, &'static str> {
    
    let result: QueryResult<Enrollment> = find_enrollment(connection,&program,&user);
    
    if result.is_err() {
        return Err(ERROR_001);
    }
  
    if result.unwrap().id > 0 {
        return Err(WARNING);   
    }

    Ok(true)
}

fn find_enrollment(connection: &MysqlConnection,program: &Program, user: &User) -> QueryResult<Enrollment> {
    
    enrollments
        .filter(program_id.eq(program.id))
        .filter(member_id.eq(user.id))
        .first(connection)
}

pub fn get_active_enrollments(connection: &MysqlConnection, criteria: EnrollmentCriteria) -> Result<Vec<User>,&'static str> {

    use crate::schema::users::dsl::*;

    let program = programs::find_by_fuzzy_id(connection,criteria.program_fuzzy_id.as_str())?;

    let result: QueryResult<Vec<User>>  = enrollments
        .inner_join(users)
        .filter(program_id.eq(program.id))
        .select(users::all_columns())
        .load(connection);
        
    if result.is_err() {
        return Err(QUERY_ERROR);
    }

    Ok(result.unwrap())
} 