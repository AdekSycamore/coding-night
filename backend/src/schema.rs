// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        author -> Varchar,
        location -> Varchar,
        title -> Varchar,
        content -> Varchar,
        link -> Varchar,
        maplink -> Varchar,
    }
}

diesel::table! {
    users (username) {
        username -> Varchar,
        password -> Varchar,
        phone -> Varchar,
        region -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
