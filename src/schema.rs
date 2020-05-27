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
    }
}

joinable!(team_members -> teams (team_id));
joinable!(team_members -> users (user_id));

allow_tables_to_appear_in_same_query!(
    team_members,
    teams,
    users,
);
