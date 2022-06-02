table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

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
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::*;

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
    use diesel_full_text_search::*;

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
    use diesel_full_text_search::*;

    packages (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        repository_url -> Text,
        total_downloads_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tsv -> TsVector,
        account_id -> Nullable<Int4>,
    }
}

joinable!(api_tokens -> accounts (account_id));
joinable!(package_versions -> packages (package_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    api_tokens,
    package_versions,
    packages,
);
