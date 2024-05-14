// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        published -> Bool,
        publish_date -> Timestamp,
    }
}
