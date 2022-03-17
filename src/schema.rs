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
