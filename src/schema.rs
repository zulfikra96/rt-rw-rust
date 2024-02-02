// @generated automatically by Diesel CLI.

diesel::table! {
    internet_services (id) {
        id -> Int4,
        #[max_length = 200]
        name -> Nullable<Varchar>,
        price -> Nullable<Numeric>,
        whitelist -> Nullable<Array<Nullable<Text>>>,
        blacklist -> Nullable<Array<Nullable<Text>>>,
        hourly -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 40]
        name -> Varchar,
        #[max_length = 6]
        token -> Nullable<Varchar>,
        role -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 64]
        ip -> Nullable<Varchar>,
        #[max_length = 64]
        mac_addr -> Nullable<Varchar>,
        #[max_length = 64]
        devices -> Nullable<Varchar>,
        internet_service_id -> Nullable<Int4>,
        enabled -> Nullable<Bool>,
        password -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        duedate -> Nullable<Timestamp>,
        mikrotik_id -> Nullable<Varchar>,
        password_plain -> Nullable<Varchar>,
    }
}

diesel::joinable!(users -> internet_services (internet_service_id));

diesel::allow_tables_to_appear_in_same_query!(
    internet_services,
    users,
);
