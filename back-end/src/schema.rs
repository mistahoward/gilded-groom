// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        details -> Text,
        timestamp -> Timestamp,
        active -> Bool,
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
    customer (id) {
        id -> Nullable<Integer>,
        name -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone_number -> Text,
        discovery_method_id -> Integer,
        created_at -> Timestamp,
        active -> Bool,
    }
}

diesel::table! {
    customer_comment (id) {
        id -> Nullable<Integer>,
        customer_id -> Integer,
        comment_id -> Integer,
    }
}

diesel::table! {
    customer_history (id) {
        id -> Nullable<Integer>,
        customer_id -> Integer,
        field_name -> Text,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        operation -> Nullable<Integer>,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    discovery_method (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        active -> Bool,
    }
}

diesel::table! {
    discovery_method_history (id) {
        id -> Nullable<Integer>,
        discovery_method_id -> Integer,
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
    service (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        price -> Float,
        base_time -> Integer,
        active -> Bool,
    }
}

diesel::table! {
    service_history (id) {
        id -> Nullable<Integer>,
        service_id -> Integer,
        field_name -> Text,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        operation -> Nullable<Integer>,
        timestamp -> Timestamp,
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
        active -> Bool,
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
diesel::joinable!(customer -> discovery_method (discovery_method_id));
diesel::joinable!(customer_comment -> comment (comment_id));
diesel::joinable!(customer_comment -> customer (customer_id));
diesel::joinable!(customer_history -> customer (customer_id));
diesel::joinable!(customer_history -> operation (operation));
diesel::joinable!(discovery_method_history -> discovery_method (discovery_method_id));
diesel::joinable!(discovery_method_history -> operation (operation));
diesel::joinable!(service_history -> operation (operation));
diesel::joinable!(service_history -> service (service_id));
diesel::joinable!(user_history -> operation (operation));
diesel::joinable!(user_history -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    comment_history,
    customer,
    customer_comment,
    customer_history,
    discovery_method,
    discovery_method_history,
    operation,
    service,
    service_history,
    user,
    user_history,
);
