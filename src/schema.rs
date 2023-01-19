// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    accounts (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        is_active -> Bool,
        is_admin -> Bool,
        has_verified_email -> Bool,
        last_login -> Nullable<Timestamptz>,
        created -> Timestamptz,
        updated -> Timestamptz,
        github_login -> Nullable<Text>,
        github_id -> Nullable<Int8>,
        avatar -> Nullable<Text>,
        slug -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    api_tokens (id) {
        id -> Int4,
        account_id -> Int4,
        token -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
        last_used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    domains (id) {
        id -> Int4,
        name -> Varchar,
        domain -> Varchar,
        price -> Nullable<Numeric>,
        owner_address -> Varchar,
        resolver -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        expiry -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    external_invitations (external_user_email, package_id) {
        external_user_email -> Text,
        invited_by_user_id -> Int4,
        package_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    owner_invitations (invited_user_id, package_id) {
        invited_user_id -> Int4,
        invited_by_user_id -> Int4,
        package_id -> Int4,
        token -> Text,
        is_transferring -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    package_collaborators (package_id, account_id) {
        package_id -> Int4,
        account_id -> Int4,
        role -> Int4,
        created_by -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    package_versions (id) {
        id -> Int4,
        package_id -> Int4,
        version -> Text,
        readme_content -> Nullable<Text>,
        license -> Nullable<Text>,
        downloads_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        rev -> Nullable<Text>,
        total_files -> Nullable<Int4>,
        total_size -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    packages (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        repository_url -> Text,
        total_downloads_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tsv -> Tsvector,
        slug -> Text,
        stars_count -> Int4,
        forks_count -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(api_tokens -> accounts (account_id));
diesel::joinable!(external_invitations -> accounts (invited_by_user_id));
diesel::joinable!(external_invitations -> packages (package_id));
diesel::joinable!(owner_invitations -> packages (package_id));
diesel::joinable!(package_collaborators -> packages (package_id));
diesel::joinable!(package_versions -> packages (package_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    accounts,
    api_tokens,
    domains,
    external_invitations,
    owner_invitations,
    package_collaborators,
    package_versions,
    packages,
    users,
);
