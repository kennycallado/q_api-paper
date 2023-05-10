// @generated automatically by Diesel CLI.

diesel::table! {
    papers (id) {
        id -> Int4,
        user_id -> Int4,
        resource_id -> Int4,
        project_id -> Int4,
        completed -> Bool,
    }
}
