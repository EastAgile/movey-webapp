-- Your SQL goes here

create table if not exists package_versions (
    id serial primary key,
    package_id integer references packages,
    version text not null,
    readme_content text,
    license text,
    downloads_count integer default 0,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

-- Auto update `updated_at` on any change
select diesel_manage_updated_at('package_versions');

-- Seed data
-- Assuming diesel package id is always 2, if broken just comment out this seed part
INSERT INTO package_versions(package_id, version, readme_content, license)
    VALUES (2, '1.0.0', 'Read me plz!', 'MIT');

INSERT INTO package_versions(package_id, version, readme_content, license)
    VALUES (2, '1.1.0', 'Read me plz! Updated', 'Apache');
