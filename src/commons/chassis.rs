use crate::models::abstract_tasks::AbstractTask;
use crate::models::enrollments::Enrollment;
use crate::models::master_plans::MasterPlan;
use crate::models::master_tasks::MasterTask;
use crate::models::notes::Note;
use crate::models::objectives::Objective;
use crate::models::observations::Observation;
use crate::models::options::Constraint;
use crate::models::programs::Program;
use crate::models::sessions::Session;
use crate::models::tasks::Task;
use crate::models::user_events::{EventRow, PlanRow, SessionPeople,ToDo};
use crate::models::user_programs::ProgramRow;
use crate::models::coach_members::MemberRow;
use crate::models::user_artifacts::NoteRow;
use crate::models::user_artifacts::BoardRow;
use crate::models::correspondences::Mailable;
use crate::models::discussions::Discussion;
use crate::models::discussion_queue::PendingFeed;

/**
 * Important: The Mutation Result might seem like a Code Duplication,
 * but is unavoidable.
 *
 * Excerpt from Graphql:Rust - Objects and Generics
 *
 * Yet another point where GraphQL and Rust differs is in how generics work.
 * In Rust, almost any type could be generic - that is, take type parameters.
 * In GraphQL, there are only two generic types: lists and non-nullables.
 * This poses a restriction on what you can expose in GraphQL from Rust:
 * no generic structs can be exposed - all type parameters must be bound.
 * For example, you can not make e.g. Result<T, E> into a GraphQL type,
 * but you can make e.g. Result<User, String> into a GraphQL type.
 */
use crate::models::users::User;

#[derive(juniper::GraphQLObject)]
pub struct QueryError {
    pub message: String,
}

#[derive(juniper::GraphQLObject)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> ValidationError {
        ValidationError {
            field: String::from(field),
            message: String::from(message),
        }
    }
}

pub struct QueryResult<T>(pub Result<T, QueryError>);

#[juniper::object(name = "ProgramsResult")]
impl QueryResult<Vec<ProgramRow>> {
    pub fn programs(&self) -> Option<&Vec<ProgramRow>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "PendingFeedResult")]
impl QueryResult<Vec<PendingFeed>> {
    pub fn feeds(&self) -> Option<&Vec<PendingFeed>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}


#[juniper::object(name = "AbstractTasksResult")]
impl QueryResult<Vec<AbstractTask>> {
    pub fn abstract_tasks(&self) -> Option<&Vec<AbstractTask>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "MasterPlansResult")]
impl QueryResult<Vec<MasterPlan>> {
    pub fn master_plans(&self) -> Option<&Vec<MasterPlan>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "MasterTasksResult")]
impl QueryResult<Vec<MasterTask>> {
    pub fn master_tasks(&self) -> Option<&Vec<MasterTask>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ObjectivesResult")]
impl QueryResult<Vec<Objective>> {
    pub fn objectives(&self) -> Option<&Vec<Objective>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "OptionsResult")]
impl QueryResult<Vec<Constraint>> {
    pub fn constraints(&self) -> Option<&Vec<Constraint>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ObservationsResult")]
impl QueryResult<Vec<Observation>> {
    pub fn observations(&self) -> Option<&Vec<Observation>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "TasksResult")]
impl QueryResult<Vec<Task>> {
    pub fn tasks(&self) -> Option<&Vec<Task>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "NotesResult")]
impl QueryResult<Vec<Note>> {
    pub fn notes(&self) -> Option<&Vec<Note>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "DiscussionsResult")]
impl QueryResult<Vec<Discussion>> {
    pub fn discussions(&self) -> Option<&Vec<Discussion>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "EnrollmentNotes")]
impl QueryResult<Vec<NoteRow>> {
    pub fn notes(&self) -> Option<&Vec<NoteRow>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "EnrollmentBoards")]
impl QueryResult<Vec<BoardRow>> {
    pub fn boards(&self) -> Option<&Vec<BoardRow>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "EventsResult")]
impl QueryResult<Vec<EventRow>> {
    pub fn sessions(&self) -> Option<&Vec<EventRow>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ActivitiesResult")]
impl QueryResult<Vec<PlanRow>> {
    pub fn planRows(&self) -> Option<&Vec<PlanRow>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ToDos")]
impl QueryResult<Vec<ToDo>> {
    pub fn todos(&self) -> Option<&Vec<ToDo>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "SessionUsers")]
impl QueryResult<Vec<SessionPeople>> {
    pub fn users(&self) -> Option<&Vec<SessionPeople>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "CoachMembers")]
impl QueryResult<Vec<MemberRow>> {
    pub fn members(&self) -> Option<&Vec<MemberRow>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "Mailables")]
impl QueryResult<Vec<Mailable>> {
    pub fn mails(&self) -> Option<&Vec<Mailable>> {
        self.0.as_ref().ok()
    }
    pub fn error(&self) -> Option<&QueryError> {
        self.0.as_ref().err()
    }
}

pub fn query_error<T>(error: diesel::result::Error) -> QueryResult<T> {
    let message: String = error.to_string();

    let e = QueryError { message: message };

    QueryResult(Err(e))
}



pub struct MutationResult<T>(pub Result<T, Vec<ValidationError>>);

#[juniper::object(name = "SessionResult")]
impl MutationResult<Session> {
    pub fn session(&self) -> Option<&Session> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "UserResult")]
impl MutationResult<User> {
    pub fn user(&self) -> Option<&User> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "AbstractTaskResult")]
impl MutationResult<AbstractTask> {
    pub fn abstract_task(&self) -> Option<&AbstractTask> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "MasterPlanResut")]
impl MutationResult<MasterPlan> {
    pub fn master_plan(&self) -> Option<&MasterPlan> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ProgramResult")]
impl MutationResult<Program> {
    pub fn program(&self) -> Option<&Program> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "EnrollmentResult")]
impl MutationResult<Enrollment> {
    pub fn enrollment(&self) -> Option<&Enrollment> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "NoteResult")]
impl MutationResult<Note> {
    pub fn note(&self) -> Option<&Note> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "DiscussionResult")]
impl MutationResult<Discussion> {
    pub fn discussion(&self) -> Option<&Discussion> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ObjectiveResult")]
impl MutationResult<Objective> {
    pub fn objective(&self) -> Option<&Objective> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "OptionResult")]
impl MutationResult<Constraint> {
    pub fn constraint(&self) -> Option<&Constraint> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "ObservationResult")]
impl MutationResult<Observation> {
    pub fn observation(&self) -> Option<&Observation> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "TaskResult")]
impl MutationResult<Task> {
    pub fn task(&self) -> Option<&Task> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "MasterTaskResult")]
impl MutationResult<MasterTask> {
    pub fn master_task(&self) -> Option<&MasterTask> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "Updates")]
impl MutationResult<String> {
    pub fn rows(&self) -> Option<&String> {
        self.0.as_ref().ok()
    }

    pub fn errors(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

pub fn service_error<T>(message: &str) -> MutationResult<T> {
    let mut v: Vec<ValidationError> = Vec::new();
    let ve = ValidationError {
        field: String::from("service"),
        message: String::from(message),
    };
    v.push(ve);
    MutationResult(Err(v))
}

pub fn mutation_error<T>(error: diesel::result::Error) -> MutationResult<T> {
    let message: String = error.to_string();

    let mut v: Vec<ValidationError> = Vec::new();
    let ve = ValidationError {
        field: String::from("service"),
        message: message,
    };
    v.push(ve);

    MutationResult(Err(v))
}
