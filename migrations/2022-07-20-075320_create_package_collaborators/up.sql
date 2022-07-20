CREATE TABLE IF NOT EXISTS package_collaborators (
    package_id integer not null references packages,
    account_id integer not null references accounts,
    role integer not null,
    created_by integer references accounts not null,
    created_at timestamp with time zone not null default now(),
    primary key (package_id, account_id)
);
