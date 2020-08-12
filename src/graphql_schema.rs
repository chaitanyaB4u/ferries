use juniper::{FieldResult, RootNode};


use crate::db_manager::MySqlConnectionPool;

use crate::models::users::{Registration, User,LoginRequest};
use crate::models::sessions::{NewSessionRequest, ChangeSessionStateRequest, Session};
use crate::models::notes::{NewNoteRequest, Note, NoteCriteria};
use crate::models::programs::{NewProgramRequest, Program, ChangeProgramStateRequest};
use crate::models::enrollments::{NewEnrollmentRequest, Enrollment,EnrollmentCriteria, PlanCriteria};
use crate::models::user_events::{get_events,get_people,EventRow,EventCriteria,SessionCriteria,SessionPeople, get_actor_sessions};
use crate::models::user_programs::{get_programs,ProgramCriteria,ProgramRow};
use crate::models::objectives::{NewObjectiveRequest,Objective};
use crate::models::tasks::{NewTaskRequest,Task};
use crate::models::options::{NewOptionRequest,Constraint};
use crate::models::observations::{NewObservationRequest,Observation};

use crate::services::users::{get_users, register, authenticate};
use crate::services::sessions::{create_session,change_session_state};
use crate::services::notes::{create_new_note, get_notes};
use crate::services::programs::{create_new_program, change_program_state};
use crate::services::enrollments::{create_new_enrollment,get_active_enrollments};
use crate::services::objectives::{create_objective, get_objectives};
use crate::services::tasks::{create_task,get_tasks};
use crate::services::options::{create_option, get_options};
use crate::services::observations::{create_observation, get_observations};

use crate::commons::chassis::{QueryResult,query_error,MutationResult,service_error,mutation_error};

#[derive(Clone)]
pub struct DBContext {
    pub db: MySqlConnectionPool,
}

impl juniper::Context for DBContext {}

pub struct QueryRoot;

#[juniper::object(Context = DBContext,description="Graph Query Root")]
impl QueryRoot {

    #[graphql(description = "Authenticate a user with email and password")]
    fn authenticate(context: &DBContext, request: LoginRequest) -> FieldResult<User> {
        let connection = context.db.get().unwrap();
        let user = authenticate(&connection,request)?;
        Ok(user)
    }

    #[graphql(description = "Get the top 100 Users")]
    fn get_users(context: &DBContext) -> Vec<User> {
        let connection = context.db.get().unwrap();
        get_users(&connection)
    }


    #[graphql(description = "Get Programs of a Coach Or Member Or Latest 10.")]
    fn get_programs(context:&DBContext, criteria:ProgramCriteria) -> QueryResult<Vec<ProgramRow>> {
         let connection = context.db.get().unwrap();
         let result = get_programs(&connection,&criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the list of members enrolled into a Program")]
    fn get_enrollments(context:&DBContext, criteria:EnrollmentCriteria) -> Vec<User> {
         let connection = context.db.get().unwrap();
         get_active_enrollments(&connection,criteria).unwrap()
    }

    #[graphql(description = "Get the list of objectives for an Enrollment")]
    fn get_objectives(context:&DBContext, criteria:PlanCriteria) -> QueryResult<Vec<Objective>> {
         let connection = context.db.get().unwrap();
         let result = get_objectives(&connection,criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the list of options for an Enrollment")]
    fn get_options(context:&DBContext, criteria:PlanCriteria) -> QueryResult<Vec<Constraint>> {
         let connection = context.db.get().unwrap();
         let result = get_options(&connection,criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the list of observations for an Enrollment")]
    fn get_observations(context:&DBContext, criteria:PlanCriteria) -> QueryResult<Vec<Observation>> {
         let connection = context.db.get().unwrap();
         let result = get_observations(&connection,criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the list of sessions for an Enrollment")]
    fn get_sessions(context:&DBContext, criteria:PlanCriteria) -> QueryResult<Vec<EventRow>> {
         let connection = context.db.get().unwrap();
         let result = get_actor_sessions(&connection,criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the Events for a User, during a period")]
    fn get_events(context:&DBContext, criteria:EventCriteria) -> QueryResult<Vec<EventRow>> { 
        let connection = context.db.get().unwrap();
        let result = get_events(&connection,criteria);

        match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the list of tasks for an Enrollment")]
    fn get_tasks(context:&DBContext, criteria:PlanCriteria) -> QueryResult<Vec<Task>> {
         let connection = context.db.get().unwrap();
         let result = get_tasks(&connection,criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the list of notes for a SessionUser")]
    fn get_notes(context:&DBContext, criteria:NoteCriteria) -> QueryResult<Vec<Note>> {
         let connection = context.db.get().unwrap();
         let result = get_notes(&connection,criteria);

         match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }

    #[graphql(description = "Get the People participating in an Event")]
    fn get_session_users(context:&DBContext, criteria:SessionCriteria) -> QueryResult<Vec<SessionPeople>> {
        let connection = context.db.get().unwrap();
        let result = get_people(&connection,criteria);

        match result {
            Ok(value) => QueryResult(Ok(value)),
            Err(e) => query_error(e),
        }
    }
}


pub struct MutationRoot;

#[juniper::object(Context = DBContext)]
impl MutationRoot {
    
    fn create_user(context: &DBContext, registration: Registration) -> FieldResult<User> {
        let connection = context.db.get().unwrap();
        let reg_result = register(&connection, &registration);

        match reg_result {
            Ok(user) => Ok(user),
            Err(e) => Err(e)?,
        }
    }


    fn create_program(context: &DBContext, new_program_request: NewProgramRequest) ->MutationResult<Program> {
       
        let errors = new_program_request.validate();
         if !errors.is_empty() {
             return MutationResult(Err(errors));
         }
 
         let connection = context.db.get().unwrap();
         let result = create_new_program(&connection, &new_program_request);
 
         match result {
             Ok(program) => MutationResult(Ok(program)),
             Err(e) => service_error(e)
         }
    }

    fn create_enrollment(context: &DBContext, new_enrollment_request: NewEnrollmentRequest) ->MutationResult<Enrollment> {
       
        let errors = new_enrollment_request.validate();
         if !errors.is_empty() {
             return MutationResult(Err(errors));
         }
 
         let connection = context.db.get().unwrap();
         let result = create_new_enrollment(&connection, &new_enrollment_request);
 
         match result {
             Ok(enrollment) => MutationResult(Ok(enrollment)),
             Err(e) => service_error(e)
         }
    }

    fn create_session(context: &DBContext, new_session_request: NewSessionRequest) -> MutationResult<Session> {

        let errors = new_session_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_session(&connection, &new_session_request);

        match result {
            Ok(session) => MutationResult(Ok(session)),
            Err(e) => service_error(e),
        }
    }

    fn create_objective(context: &DBContext, new_objective_request: NewObjectiveRequest) -> MutationResult<Objective> {

        let errors = new_objective_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_objective(&connection, &new_objective_request);

        match result {
            Ok(objective) => MutationResult(Ok(objective)),
            Err(e) => mutation_error(e),
        }
    }

    fn create_option(context: &DBContext, new_option_request: NewOptionRequest) -> MutationResult<Constraint> {

        let errors = new_option_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_option(&connection, &new_option_request);

        match result {
            Ok(option) => MutationResult(Ok(option)),
            Err(e) => mutation_error(e),
        }
    }

    fn create_observation(context: &DBContext, new_observation_request: NewObservationRequest) -> MutationResult<Observation> {

        let errors = new_observation_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_observation(&connection, &new_observation_request);

        match result {
            Ok(option) => MutationResult(Ok(option)),
            Err(e) => mutation_error(e),
        }
    }

    fn create_task(context: &DBContext, new_task_request: NewTaskRequest) -> MutationResult<Task> {

        let errors = new_task_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_task(&connection, &new_task_request);

        match result {
            Ok(task) => MutationResult(Ok(task)),
            Err(e) => mutation_error(e),
        }
    }

    fn alter_session_state(context: &DBContext, request: ChangeSessionStateRequest) -> MutationResult<Session> {
        let connection = context.db.get().unwrap();
        let result = change_session_state(&connection, &request);
        
        match result {
            Ok(session) => MutationResult(Ok(session)),
            Err(e) => service_error(e),
        }
    }


    fn alter_program_state(context: &DBContext, request: ChangeProgramStateRequest) -> MutationResult<String> {
        let connection = context.db.get().unwrap();
        let result = change_program_state(&connection, &request);

        match result {
            Ok(rows) => MutationResult(Ok(String::from("Ok"))),
            Err(e) => service_error(e),
        }
    }
    
    fn create_note(context: &DBContext, new_note_request: NewNoteRequest) -> MutationResult<Note> {

        let errors = new_note_request.validate();
        if !errors.is_empty() {
            return MutationResult(Err(errors));
        }

        let connection = context.db.get().unwrap();
        let result = create_new_note(&connection, &new_note_request);

        match result {
            Ok(note) => MutationResult(Ok(note)),
            Err(e) => mutation_error(e),
        }
    }
}

pub type GQSchema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_gq_schema() -> GQSchema {
    GQSchema::new(QueryRoot {}, MutationRoot {})
}
