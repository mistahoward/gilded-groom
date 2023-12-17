// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        name -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        salt -> Text,
        created_at -> Text,
        last_login -> Text,
    }
}
