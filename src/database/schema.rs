// @generated automatically by Diesel CLI.

diesel::table! {
    paper_answers (id) {
        id -> Int4,
        paper_id -> Int4,
        answer_id -> Int4,
    }
}

diesel::table! {
    papers (id) {
        id -> Int4,
        user_id -> Int4,
        resource_id -> Int4,
        project_id -> Int4,
        completed -> Bool,
    }
}

diesel::joinable!(paper_answers -> papers (paper_id));

diesel::allow_tables_to_appear_in_same_query!(
    paper_answers,
    papers,
);
