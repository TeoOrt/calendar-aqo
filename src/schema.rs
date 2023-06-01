// @generated automatically by Diesel CLI.

diesel::table! {
    calendar (id) {
        id -> Int4,
        time -> Text,
        date -> Text,
        description -> Text,
    }
}
