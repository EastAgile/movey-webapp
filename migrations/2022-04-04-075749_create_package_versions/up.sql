-- Your SQL goes here

create table if not exists package_versions (
    id serial primary key,
    package_id integer not null references packages,
    version text not null,
    readme_content text,
    license text,
    downloads_count integer not null default 0,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

-- Auto update `updated_at` on any change
select diesel_manage_updated_at('package_versions');
