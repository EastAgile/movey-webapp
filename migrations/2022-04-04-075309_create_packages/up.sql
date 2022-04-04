-- Your SQL goes here

create table if not exists packages (
    id serial primary key,
    name text not null unique,
    description text not null,
    repository_url text not null,
    total_downloads_count integer default 0,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

-- Auto update `updated_at` on any change
select diesel_manage_updated_at('packages');

-- Seed data
INSERT INTO packages(name, description, repository_url)
    VALUES ('rand', 'Random number generators and other randomness functionality.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, description, repository_url)
    VALUES ('diesel', 'A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL', 'https://github.com/diesel-rs/diesel');
