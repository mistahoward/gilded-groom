// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        details -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    comment_history (id) {
        id -> Nullable<Integer>,
        comment_id -> Integer,
        field_name -> Text,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        operation -> Nullable<Integer>,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    operation (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        name -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        salt -> Text,
        created_at -> Integer,
        last_login -> Integer,
    }
}

diesel::table! {
    user_history (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        field_name -> Text,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        operation -> Nullable<Integer>,
        timestamp -> Timestamp,
    }
}

diesel::joinable!(comment -> user (user_id));
diesel::joinable!(comment_history -> comment (comment_id));
diesel::joinable!(comment_history -> operation (operation));
diesel::joinable!(user_history -> operation (operation));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    comment_history,
    operation,
    user,
    user_history,
);
