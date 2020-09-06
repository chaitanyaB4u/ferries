table! {
    abstract_tasks (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    coaches (id) {
        id -> Varchar,
        user_id -> Varchar,
        full_name -> Varchar,
        email -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    enrollments (id) {
        id -> Varchar,
        program_id -> Varchar,
        member_id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
        is_new -> Bool,
    }
}

table! {
    master_plans (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    master_task_links (id) {
        id -> Varchar,
        source_task_id -> Varchar,
        target_task_id -> Varchar,
        lead_time -> Integer,
        buffer_time -> Integer,
        coordinates -> Text,
        priority -> Integer,
        is_forward -> Bool,
    }
}

table! {
    master_tasks (id) {
        id -> Varchar,
        master_plan_id -> Varchar,
        abstract_task_id -> Varchar,
        duration -> Integer,
        min -> Integer,
        max -> Integer,
        task_type -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    objectives (id) {
        id -> Varchar,
        enrollment_id -> Varchar,
        duration -> Integer,
        original_start_date -> Datetime,
        original_end_date -> Datetime,
        revised_start_date -> Nullable<Datetime>,
        revised_end_date -> Nullable<Datetime>,
        actual_start_date -> Nullable<Datetime>,
        actual_end_date -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        description -> Nullable<Text>,
        closing_notes -> Nullable<Text>,
    }
}

table! {
    observations (id) {
        id -> Varchar,
        enrollment_id -> Varchar,
        description -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    options (id) {
        id -> Varchar,
        enrollment_id -> Varchar,
        description -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    program_plans (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Text,
        master_plan_id -> Varchar,
        program_id -> Varchar,
    }
}

table! {
    programs (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        active -> Bool,
        coach_name -> Varchar,
        coach_id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    session_files (id) {
        id -> Varchar,
        session_note_id -> Varchar,
        file_name -> Varchar,
        file_path -> Varchar,
        file_type -> Nullable<Varchar>,
        file_size -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    session_notes (id) {
        id -> Varchar,
        session_id -> Varchar,
        created_by_id -> Varchar,
        session_user_id -> Varchar,
        description -> Text,
        remind_at -> Nullable<Datetime>,
        is_private -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    session_users (id) {
        id -> Varchar,
        session_id -> Varchar,
        user_id -> Varchar,
        user_type -> Varchar,
    }
}

table! {
    sessions (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        program_id -> Varchar,
        enrollment_id -> Varchar,
        people -> Nullable<Text>,
        duration -> Integer,
        original_start_date -> Datetime,
        original_end_date -> Datetime,
        revised_start_date -> Nullable<Datetime>,
        revised_end_date -> Nullable<Datetime>,
        offered_start_date -> Nullable<Datetime>,
        offered_end_date -> Nullable<Datetime>,
        is_ready -> Bool,
        actual_start_date -> Nullable<Datetime>,
        actual_end_date -> Nullable<Datetime>,
        cancelled_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        closing_notes -> Nullable<Text>,
    }
}

table! {
    task_links (id) {
        id -> Varchar,
        source_task_id -> Varchar,
        target_task_id -> Varchar,
        lead_time -> Integer,
        buffer_time -> Integer,
        coordinates -> Text,
        priority -> Integer,
        is_forward -> Bool,
    }
}

table! {
    tasks (id) {
        id -> Varchar,
        enrollment_id -> Varchar,
        actor_id -> Varchar,
        name -> Varchar,
        duration -> Integer,
        min -> Integer,
        max -> Integer,
        original_start_date -> Datetime,
        original_end_date -> Datetime,
        revised_start_date -> Nullable<Datetime>,
        revised_end_date -> Nullable<Datetime>,
        offered_start_date -> Nullable<Datetime>,
        offered_end_date -> Nullable<Datetime>,
        actual_start_date -> Nullable<Datetime>,
        actual_end_date -> Nullable<Datetime>,
        locked -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        description -> Nullable<Text>,
        closing_notes -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        full_name -> Varchar,
        email -> Varchar,
        blocked -> Bool,
        user_type -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

joinable!(coaches -> users (user_id));
joinable!(enrollments -> programs (program_id));
joinable!(enrollments -> users (member_id));
joinable!(master_tasks -> abstract_tasks (abstract_task_id));
joinable!(master_tasks -> master_plans (master_plan_id));
joinable!(objectives -> enrollments (enrollment_id));
joinable!(observations -> enrollments (enrollment_id));
joinable!(options -> enrollments (enrollment_id));
joinable!(program_plans -> master_plans (master_plan_id));
joinable!(program_plans -> programs (program_id));
joinable!(programs -> coaches (coach_id));
joinable!(session_files -> session_notes (session_note_id));
joinable!(session_notes -> session_users (session_user_id));
joinable!(session_notes -> sessions (session_id));
joinable!(session_notes -> users (created_by_id));
joinable!(session_users -> sessions (session_id));
joinable!(session_users -> users (user_id));
joinable!(sessions -> enrollments (enrollment_id));
joinable!(sessions -> programs (program_id));
joinable!(tasks -> enrollments (enrollment_id));
joinable!(tasks -> users (actor_id));

allow_tables_to_appear_in_same_query!(
    abstract_tasks,
    coaches,
    enrollments,
    master_plans,
    master_task_links,
    master_tasks,
    objectives,
    observations,
    options,
    program_plans,
    programs,
    session_files,
    session_notes,
    session_users,
    sessions,
    task_links,
    tasks,
    users,
);
