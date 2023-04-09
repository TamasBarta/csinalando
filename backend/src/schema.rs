// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        uid -> Text,
        title -> Text,
        completed -> Bool,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}
