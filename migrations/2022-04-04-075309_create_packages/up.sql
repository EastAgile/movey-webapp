-- Your SQL goes here

create table if not exists packages (
    id serial primary key,
    name text not null unique,
    description text not null,
    repository_url text not null,
    total_downloads_count integer not null default 0,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

-- Auto update `updated_at` on any change
select diesel_manage_updated_at('packages');
