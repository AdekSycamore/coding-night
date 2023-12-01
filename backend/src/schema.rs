// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        task -> Varchar,
        done -> Bool,
    }
}

diesel::table! {
    users (username) {
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
