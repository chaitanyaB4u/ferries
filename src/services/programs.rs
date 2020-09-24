use diesel::prelude::*;

use crate::models::coaches::Coach;
use crate::models::programs::{ChangeProgramStateRequest, NewProgram, NewProgramRequest, Program, ProgramTargetState};

use crate::schema::coaches::dsl::*;
use crate::schema::programs::dsl::*;

const INVALID_PROGRAM: &'static str = "Invalid Program Id. Error:001.";
const PROGRAM_CREATION_ERROR: &'static str = "Program Creation. Error:002";
const INVALID_COACH: &'static str = "Invalid Coach Fuzzy Id. Error:003";
const PROGRAM_STATE_CHANGE_ERROR: &'static str = "Unable to change the state of the program";
const PROGRAM_SAME_STATE_ERROR: &'static str = "Program is already in the target state.";

pub fn find(connection: &MysqlConnection, the_id: &str) -> Result<Program, &'static str> {
    use crate::schema::programs::dsl::id;

    let result = programs.filter(id.eq(the_id)).first(connection);

    if result.is_err() {
        return Err(INVALID_PROGRAM);
    }

    Ok(result.unwrap())
}

pub fn create_new_program(connection: &MysqlConnection, request: &NewProgramRequest) -> Result<Program, &'static str> {
    //Finding coach with fuzzy_id
    let coach = get_coach(connection, request)?;

    //Transform result into new_program
    let new_program = NewProgram::from(request, &coach);

    insert_program(connection, &new_program)
}

/**
 * The id of coach and user_id will be the same. The Coaches table is a
 * convenience for avoiding self-join.
 */
fn get_coach(connection: &MysqlConnection, request: &NewProgramRequest) -> Result<Coach, &'static str> {
    use crate::schema::coaches::dsl::id;

    let the_coach_id = request.coach_id.as_str();

    let coach_result = coaches.filter(id.eq(the_coach_id)).first(connection);

    if coach_result.is_err() {
        return Err(INVALID_COACH);
    }

    Ok(coach_result.unwrap())
}

fn insert_program(connection: &MysqlConnection, new_program: &NewProgram) -> Result<Program, &'static str> {
    let result = diesel::insert_into(programs).values(new_program).execute(connection);

    if result.is_err() {
        return Err(PROGRAM_CREATION_ERROR);
    }

    find(connection, new_program.id.as_str())
}

pub fn change_program_state(connection: &MysqlConnection, request: &ChangeProgramStateRequest) -> Result<usize, &'static str> {
    let program = &find(connection, request.id.as_str())?;

    validate_target_state(program, request)?;

    let result = match request.target_state {
        ProgramTargetState::ACTIVATE => diesel::update(program).set(active.eq(true)).execute(connection),
        ProgramTargetState::DEACTIVATE => diesel::update(program).set(active.eq(false)).execute(connection),
    };

    if result.is_err() {
        return Err(PROGRAM_STATE_CHANGE_ERROR);
    }

    Ok(result.unwrap())
}

fn validate_target_state(program: &Program, request: &ChangeProgramStateRequest) -> Result<bool, &'static str> {
    if program.active && request.target_state == ProgramTargetState::ACTIVATE {
        return Err(PROGRAM_SAME_STATE_ERROR);
    }
    if !program.active && request.target_state == ProgramTargetState::DEACTIVATE {
        return Err(PROGRAM_SAME_STATE_ERROR);
    }

    Ok(true)
}
