// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (permission_id) {
        #[max_length = 50]
        permission_id -> Varchar,
        #[max_length = 100]
        permission_name -> Varchar,
        #[max_length = 100]
        permission_code -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    remember_tokens (id) {
        id -> Bigint,
        #[max_length = 50]
        user_id -> Varchar,
        #[max_length = 255]
        token_hash -> Varchar,
        #[max_length = 255]
        device_info -> Nullable<Varchar>,
        #[max_length = 50]
        ip_address -> Nullable<Varchar>,
        expire_time -> Nullable<Datetime>,
    }
}

diesel::table! {
    role_permissions (id) {
        id -> Bigint,
        #[max_length = 50]
        role_id -> Varchar,
        #[max_length = 50]
        permission_id -> Varchar,
        created_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    roles (role_id) {
        #[max_length = 50]
        role_id -> Varchar,
        #[max_length = 100]
        role_name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        status -> Nullable<Tinyint>,
        created_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Bigint,
        #[max_length = 50]
        user_id -> Varchar,
        #[max_length = 50]
        role_id -> Varchar,
        created_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Bigint,
        #[max_length = 50]
        user_id -> Varchar,
        #[max_length = 36]
        session_id -> Char,
        #[max_length = 255]
        browser_info -> Nullable<Varchar>,
        #[max_length = 50]
        ip_address -> Nullable<Varchar>,
        login_time -> Nullable<Datetime>,
        expire_time -> Nullable<Datetime>,
        is_active -> Nullable<Tinyint>,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        #[max_length = 50]
        user_id -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        status -> Nullable<Tinyint>,
        created_at -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    remember_tokens,
    role_permissions,
    roles,
    user_roles,
    user_sessions,
    users,
);
