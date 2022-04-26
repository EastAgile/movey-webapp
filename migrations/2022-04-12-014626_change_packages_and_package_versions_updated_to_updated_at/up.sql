-- Your SQL goes here

ALTER TABLE packages RENAME COLUMN created TO created_at;
ALTER TABLE packages RENAME COLUMN updated TO updated_at;

ALTER TABLE package_versions RENAME COLUMN created TO created_at;
ALTER TABLE package_versions RENAME COLUMN updated TO updated_at;
