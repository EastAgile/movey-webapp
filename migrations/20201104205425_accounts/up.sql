-- Creates a accounts table, along with some associated helpers.

create or replace function update_timestamp() returns trigger as $$
begin
    new.updated = now();
    return new;
end;
$$ language 'plpgsql';

create table if not exists accounts (
    id serial primary key,
    name text not null,
    email text not null unique,
    password text not null,
    is_active boolean not null default true,
    is_admin boolean not null default false,
    has_verified_email boolean not null default false,
    last_login timestamp with time zone,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

create unique index accounts_unique_lower_email_idx on accounts (lower(email));

create trigger user_updated before insert or update on accounts
for each row execute procedure update_timestamp();
