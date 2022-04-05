table! {
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
    package_versions (id) {
        id -> Int4,
        package_id -> Int4,
        version -> Text,
        readme_content -> Nullable<Text>,
        license -> Nullable<Text>,
        downloads_count -> Int4,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    packages (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        repository_url -> Text,
        total_downloads_count -> Int4,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

joinable!(package_versions -> packages (package_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    package_versions,
    packages,
);
