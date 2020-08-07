table! {
    coaches (id) {
        id -> Integer,
        user_id -> Integer,
        full_name -> Varchar,
        email -> Varchar,
        fuzzy_id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    enrollments (id) {
        id -> Integer,
        program_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        member_id -> Integer,
        fuzzy_id -> Varchar,
    }
}

table! {
    objectives (id) {
        id -> Integer,
        enrollment_id -> Integer,
        fuzzy_id -> Varchar,
        duration -> Integer,
        original_start_date -> Datetime,
        original_end_date -> Datetime,
        revised_start_date -> Nullable<Datetime>,
        revised_end_date -> Nullable<Datetime>,
        actual_start_date -> Nullable<Datetime>,
        actual_end_date -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    observations (id) {
        id -> Integer,
        enrollment_id -> Integer,
        fuzzy_id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    options (id) {
        id -> Integer,
        enrollment_id -> Integer,
        fuzzy_id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    programs (id) {
        id -> Integer,
        name -> Varchar,
        active -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        fuzzy_id -> Varchar,
        description -> Nullable<Text>,
        coach_name -> Varchar,
        coach_id -> Integer,
    }
}

table! {
    session_boards (id) {
        id -> Integer,
        fuzzy_id -> Varchar,
        session_id -> Integer,
        file_name -> Varchar,
        file_path -> Varchar,
        created_by_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        session_user_id -> Integer,
        board_tag -> Varchar,
    }
}

table! {
    session_files (id) {
        id -> Integer,
        fuzzy_id -> Varchar,
        session_note_id -> Integer,
        file_name -> Varchar,
        file_path -> Varchar,
        file_type -> Nullable<Varchar>,
        file_size -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    session_links (id) {
        id -> Integer,
        source_session_id -> Integer,
        target_session_id -> Integer,
        lead_time -> Integer,
        buffer_time -> Integer,
        coordinates -> Text,
        priority -> Integer,
        is_forward -> Bool,
    }
}

table! {
    session_notes (id) {
        id -> Integer,
        fuzzy_id -> Varchar,
        session_id -> Integer,
        description -> Text,
        remind_at -> Nullable<Datetime>,
        created_by_id -> Integer,
        is_private -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        session_user_id -> Integer,
    }
}

table! {
    session_users (id) {
        id -> Integer,
        fuzzy_id -> Varchar,
        session_id -> Integer,
        user_id -> Integer,
        user_type -> Varchar,
    }
}

table! {
    session_visits (id) {
        id -> Integer,
        session_id -> Integer,
        user_id -> Integer,
        joined_at -> Datetime,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        program_id -> Integer,
        name -> Varchar,
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
        created_at -> Datetime,
        updated_at -> Datetime,
        description -> Nullable<Text>,
        fuzzy_id -> Varchar,
        people -> Nullable<Text>,
        cancelled_at -> Nullable<Datetime>,
        enrollment_id -> Integer,
    }
}

table! {
    task_links (id) {
        id -> Integer,
        source_task_id -> Integer,
        target_task_id -> Integer,
        lead_time -> Integer,
        buffer_time -> Integer,
        coordinates -> Text,
        priority -> Integer,
        is_forward -> Bool,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        enrollment_id -> Integer,
        fuzzy_id -> Varchar,
        name -> Varchar,
        duration -> Integer,
        min -> Integer,
        max -> Integer,
        actor_id -> Integer,
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
    }
}

table! {
    team_members (id) {
        id -> Integer,
        team_id -> Integer,
        user_id -> Integer,
        blocked -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    teams (id) {
        id -> Integer,
        name -> Varchar,
        fuzzy_id -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Integer,
        full_name -> Varchar,
        email -> Varchar,
        fuzzy_id -> Varchar,
        blocked -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        user_type -> Varchar,
    }
}

joinable!(coaches -> users (user_id));
joinable!(enrollments -> programs (program_id));
joinable!(enrollments -> users (member_id));
joinable!(objectives -> enrollments (enrollment_id));
joinable!(observations -> enrollments (enrollment_id));
joinable!(options -> enrollments (enrollment_id));
joinable!(programs -> coaches (coach_id));
joinable!(session_boards -> session_users (session_user_id));
joinable!(session_boards -> sessions (session_id));
joinable!(session_boards -> users (created_by_id));
joinable!(session_files -> session_notes (session_note_id));
joinable!(session_notes -> session_users (session_user_id));
joinable!(session_notes -> sessions (session_id));
joinable!(session_notes -> users (created_by_id));
joinable!(session_users -> sessions (session_id));
joinable!(session_users -> users (user_id));
joinable!(session_visits -> sessions (session_id));
joinable!(session_visits -> users (user_id));
joinable!(sessions -> enrollments (enrollment_id));
joinable!(sessions -> programs (program_id));
joinable!(tasks -> enrollments (enrollment_id));
joinable!(tasks -> users (actor_id));
joinable!(team_members -> teams (team_id));
joinable!(team_members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    coaches,
    enrollments,
    objectives,
    observations,
    options,
    programs,
    session_boards,
    session_files,
    session_links,
    session_notes,
    session_users,
    session_visits,
    sessions,
    task_links,
    tasks,
    team_members,
    teams,
    users,
);
