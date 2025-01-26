// @generated automatically by Diesel CLI.

diesel::table! {
    conversion_transactions (id) {
        id -> Int4,
        user_id -> Int4,
        source_size -> Int8,
        target_size -> Int8,
        credit_used -> Int8,
        conversion_type -> Varchar,
        source_type -> Varchar,
        target_type -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    credit_transactions (id) {
        id -> Int4,
        user_id -> Int4,
        transaction_type -> Varchar,
        amount -> Int8,
        source -> Varchar,
        transaction_id -> Varchar,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        credit -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_login -> Nullable<Timestamptz>,
        session_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(conversion_transactions -> users (user_id));
diesel::joinable!(credit_transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversion_transactions,
    credit_transactions,
    users,
);
