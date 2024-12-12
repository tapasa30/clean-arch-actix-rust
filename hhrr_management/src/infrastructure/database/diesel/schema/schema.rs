// @generated automatically by Diesel CLI.

diesel::table! {
    demo_models (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        is_published -> Bool,
    }
}
