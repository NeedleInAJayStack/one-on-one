// @generated automatically by Diesel CLI.

diesel::table! {
    meetings (id) {
        id -> Int4,
        one_on_one_id -> Int4,
        date -> Timestamp,
        manager_topics -> Nullable<Text>,
        employee_topics -> Nullable<Text>,
        notes -> Nullable<Text>,
        action_items -> Nullable<Text>,
    }
}

diesel::table! {
    one_on_ones (id) {
        id -> Int4,
        manager_id -> Int4,
        employee_id -> Nullable<Int4>,
    }
}

diesel::table! {
    survey_responses (id) {
        id -> Int4,
        survey_id -> Int4,
        date -> Timestamp,
        question -> Text,
    }
}

diesel::table! {
    surveys (id) {
        id -> Int4,
        one_on_one_id -> Int4,
        question -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::joinable!(meetings -> one_on_ones (one_on_one_id));
diesel::joinable!(survey_responses -> surveys (survey_id));
diesel::joinable!(surveys -> one_on_ones (one_on_one_id));

diesel::allow_tables_to_appear_in_same_query!(
    meetings,
    one_on_ones,
    survey_responses,
    surveys,
    users,
);
