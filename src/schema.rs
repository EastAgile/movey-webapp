table! {
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
    }
}

table! {
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

table! {
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

table! {
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

table! {
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

table! {
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
        account_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};

    pending_invitations (pending_user_email, package_id) {
        pending_user_email -> Text,
        invited_by_user_id -> Int4,
        package_id -> Int4,
        token -> Text,
        created_at -> Timestamp,
    }
}

joinable!(api_tokens -> accounts (account_id));
joinable!(owner_invitations -> packages (package_id));
joinable!(package_collaborators -> packages (package_id));
joinable!(package_versions -> packages (package_id));
joinable!(packages -> accounts (account_id));
joinable!(pending_invitations -> accounts (invited_by_user_id));
joinable!(pending_invitations -> packages (package_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    api_tokens,
    owner_invitations,
    package_collaborators,
    package_versions,
    packages,
    pending_invitations,
);
