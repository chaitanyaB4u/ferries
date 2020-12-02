use diesel::prelude::*;

use crate::models::coaches::Coach;
use crate::models::programs::{AssociateCoachRequest, ChangeProgramStateRequest, NewProgram, NewProgramRequest, Program, ProgramCoach, ProgramTargetState};

use crate::services::users::{find_coach_by_email, find_coach_by_id};

use crate::schema::coaches::dsl::*;
use crate::schema::programs;
use crate::schema::programs::dsl::*;

const INVALID_PROGRAM: &str = "Invalid Program Id. Error:001.";
const PROGRAM_CREATION_ERROR: &str = "Program Creation. Error:002";

const PROGRAM_STATE_CHANGE_ERROR: &str = "Unable to change the state of the program";
const PROGRAM_SAME_STATE_ERROR: &str = "Program is already in the target state.";

pub fn find(connection: &MysqlConnection, the_id: &str) -> Result<Program, &'static str> {
    let result = programs.filter(programs::id.eq(the_id)).first(connection);

    if result.is_err() {
        return Err(INVALID_PROGRAM);
    }

    Ok(result.unwrap())
}

/**
 * The id of coach and user_id will be the same. The Coaches table is a
 * convenience for avoiding self-join.
 *
 * 29-Nov:
 * The program will be the parent program through this route
 *
 */
pub fn create_new_program(connection: &MysqlConnection, request: &NewProgramRequest) -> Result<Program, &'static str> {
    //Finding coach with fuzzy_id
    let coach = find_coach_by_id(connection, request.coach_id.as_str())?;

    //Transform result into new_program
    let new_program = NewProgram::from_request(request, &coach);

    insert_program(connection, &new_program)
}

/**
 * Spwan a new Program from the Parent Program when associating a another coach
 */
pub fn associate_coach(connection: &MysqlConnection, request: &AssociateCoachRequest) -> Result<Program, &'static str> {
    let coach = find_coach_by_email(connection, request.peer_coach_email.as_str())?;

    let program = find(connection, request.program_id.as_str())?;
    let new_program = NewProgram::from_parent_program(&program, &coach);

    insert_program(connection, &new_program)
}

/**
 *
 * The given program_id may either a parent or a spawned one.
 *
 * Return the list of all the associated coaches for the program.
 */

pub fn get_peer_coaches(connection: &MysqlConnection, the_program_id: &str) -> Result<Vec<ProgramCoach>, diesel::result::Error> {
    let program = programs.filter(programs::id.eq(the_program_id)).first::<Program>(connection)?;
    let root_program_id = program.parent_program_id.unwrap_or(program.id);
    let peer_coaches: Vec<ProgramCoach> = programs
        .inner_join(coaches)
        .filter(parent_program_id.eq(root_program_id))
        .load(connection)?
        .into_iter()
        .map(|tuple: (Program, Coach)| ProgramCoach { program: tuple.0, coach: tuple.1 })
        .collect();

    Ok(peer_coaches)
}

fn insert_program(connection: &MysqlConnection, new_program: &NewProgram) -> Result<Program, &'static str> {
    let result = diesel::insert_into(programs).values(new_program).execute(connection);

    if result.is_err() {
        return Err(PROGRAM_CREATION_ERROR);
    }

    find(connection, new_program.id.as_str())
}

/***
 * When we change the state of the Parent Program,
 * we need to change state of all the Peer Programs as well.
 *
 * The state change shall be permitted only from the parent program.
 */
pub fn change_program_state(connection: &MysqlConnection, request: &ChangeProgramStateRequest) -> Result<usize, &'static str> {
    let program = &find(connection, request.id.as_str())?;
    validate_target_state(program, request)?;

    let target_programs = programs.filter(parent_program_id.eq(request.id.as_str()));

    let result = match request.target_state {
        ProgramTargetState::ACTIVATE => diesel::update(target_programs).set(active.eq(true)).execute(connection),
        ProgramTargetState::DEACTIVATE => diesel::update(target_programs).set(active.eq(false)).execute(connection),
    };

    if result.is_err() {
        return Err(PROGRAM_STATE_CHANGE_ERROR);
    }

    Ok(result.unwrap())
}

fn validate_target_state(program: &Program, request: &ChangeProgramStateRequest) -> Result<bool, &'static str> {
    if !program.is_parent {
        return Err(PROGRAM_STATE_CHANGE_ERROR);
    }
    if program.active && request.target_state == ProgramTargetState::ACTIVATE {
        return Err(PROGRAM_SAME_STATE_ERROR);
    }
    if !program.active && request.target_state == ProgramTargetState::DEACTIVATE {
        return Err(PROGRAM_SAME_STATE_ERROR);
    }

    Ok(true)
}
