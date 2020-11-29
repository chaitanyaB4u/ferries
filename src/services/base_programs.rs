use diesel::prelude::*;
use diesel::result::Error;

use crate::models::base_program_coaches::{AssociateCoachRequest, NewBaseProgramCoach};
use crate::models::base_programs::{BaseProgram, NewBaseProgram, NewBaseProgramRequest};
use crate::models::programs::{ChangeProgramStateRequest, NewProgramRequest, Program, ProgramTargetState};

use crate::services::programs::{change_program_state, create_new_program};

use crate::schema::programs;
use crate::schema::programs::dsl::*;

use crate::schema::base_program_coaches;
use crate::schema::base_program_coaches::dsl::*;

use crate::schema::base_programs;
use crate::schema::base_programs::dsl::*;

const INVALID_PROGRAM: &str = "Invalid Program Id. Error:001.";
const CREATION_ERROR: &str = "Program Creation. Error:002";
const COACH_ASSOCIATION_ERROR: &str = "Unable to associate the coach to the base program. Error:003";
const PRIOR_ASSOCIATION_ERROR: &str = "The coach is associated already to this base program. Error:004";

const STATE_CHANGE_ERROR: &str = "Unable to change the state of the program";
const SAME_STATE_ERROR: &str = "Program is already in the target state.";

pub fn find(connection: &MysqlConnection, the_id: &str) -> Result<BaseProgram, &'static str> {
    let result = base_programs.filter(base_programs::id.eq(the_id)).first(connection);

    if result.is_err() {
        return Err(INVALID_PROGRAM);
    }

    Ok(result.unwrap())
}

pub fn create_base_program(connection: &MysqlConnection, request: &NewBaseProgramRequest) -> Result<BaseProgram, &'static str> {
    let new_base_program = NewBaseProgram::from(request);
    insert_base_program(connection, &new_base_program)?;

    let associate_coach_request = AssociateCoachRequest {
        coach_id: request.coach_id.to_owned(),
        base_program_id: new_base_program.id.to_owned(),
        admin_coach_id: request.coach_id.to_owned(),
        is_admin: true,
    };

    associate_coach(connection, &associate_coach_request)?;

    find(connection, new_base_program.id.as_str())
}

/**
 * 1. Check if the coach is already associated
 * 2. Associate the coach to the base_program
 * 3. Spawn a private program for the Coach
 * 4. Sync the activation state of the Spawned Program
 */
pub fn associate_coach(connection: &MysqlConnection, request: &AssociateCoachRequest) -> Result<String, &'static str> {
    let base_program_coach = NewBaseProgramCoach::from(request);

    gate_prior_association(connection, &base_program_coach)?;

    insert_coach(connection, &base_program_coach)?;

    let base_program = find(connection, base_program_coach.base_program_id.as_str())?;

    let new_program_request = NewProgramRequest {
        coach_id: request.coach_id.to_owned(),
        name: base_program.name.to_owned(),
        description: base_program.description.to_owned().unwrap(),
        is_private: true,
        base_program_id: Some(base_program.id.to_owned()),
        genre_id: Some(base_program.genre_id.to_owned()),
    };

    let program = create_new_program(connection, &new_program_request)?;

    sync_sub_program_state(connection, &base_program, &program)?;

    Ok(program.id)
}

fn sync_sub_program_state(connection: &MysqlConnection, base_program: &BaseProgram, sub_program: &Program) -> Result<usize, &'static str> {
    
    if !base_program.active {
        return Ok(0)
    }

    let state_change_request = ChangeProgramStateRequest {
        id: sub_program.id.to_owned(),
        target_state: ProgramTargetState::ACTIVATE,
    };

    change_program_state(connection, &state_change_request)
}

fn gate_prior_association(connection: &MysqlConnection, program_coach: &NewBaseProgramCoach) -> Result<(), &'static str> {
    let result: Result<String, Error> = programs
        .filter(programs::coach_id.eq(program_coach.coach_id.as_str()))
        .filter(programs::base_program_id.eq(program_coach.base_program_id.as_str()))
        .select(programs::coach_id)
        .first(connection);

    if result.is_ok() {
        return Err(PRIOR_ASSOCIATION_ERROR);
    }

    let result: Result<String, Error> = base_program_coaches
        .filter(base_program_coaches::coach_id.eq(program_coach.coach_id.as_str()))
        .filter(base_program_coaches::base_program_id.eq(program_coach.base_program_id.as_str()))
        .select(base_program_coaches::coach_id)
        .first(connection);

    if result.is_ok() {
        return Err(PRIOR_ASSOCIATION_ERROR);
    }

    Ok(())
}

fn insert_base_program(connection: &MysqlConnection, new_base_program: &NewBaseProgram) -> Result<usize, &'static str> {
    let result = diesel::insert_into(base_programs).values(new_base_program).execute(connection);

    if result.is_err() {
        return Err(CREATION_ERROR);
    }

    Ok(result.unwrap())
}

fn insert_coach(connection: &MysqlConnection, program_coach: &NewBaseProgramCoach) -> Result<usize, &'static str> {
    let result = diesel::insert_into(base_program_coaches).values(program_coach).execute(connection);

    if result.is_err() {
        return Err(COACH_ASSOCIATION_ERROR);
    }

    Ok(result.unwrap())
}

pub fn change_base_program_state(connection: &MysqlConnection, request: &ChangeProgramStateRequest) -> Result<usize, &'static str> {
    let program = &find(connection, request.id.as_str())?;

    validate_target_state(program, request)?;

    let result = match request.target_state {
        ProgramTargetState::ACTIVATE => diesel::update(program).set(base_programs::active.eq(true)).execute(connection),
        ProgramTargetState::DEACTIVATE => diesel::update(program).set(base_programs::active.eq(false)).execute(connection),
    };

    if result.is_err() {
        return Err(STATE_CHANGE_ERROR);
    }

    Ok(result.unwrap())
}

fn validate_target_state(program: &BaseProgram, request: &ChangeProgramStateRequest) -> Result<bool, &'static str> {
    if program.active && request.target_state == ProgramTargetState::ACTIVATE {
        return Err(SAME_STATE_ERROR);
    }
    if !program.active && request.target_state == ProgramTargetState::DEACTIVATE {
        return Err(SAME_STATE_ERROR);
    }

    Ok(true)
}

pub fn get_latest_base_programs(connection: &MysqlConnection) -> Result<Vec<BaseProgram>, diesel::result::Error> {
    base_programs
        .order_by(base_programs::created_at.asc())
        .filter(base_programs::active.eq(true))
        .limit(10)
        .load(connection)
}
