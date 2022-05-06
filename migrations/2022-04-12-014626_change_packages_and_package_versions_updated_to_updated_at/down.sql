-- This file should undo anything in `up.sql`
ALTER TABLE packages RENAME COLUMN created_at TO created;
ALTER TABLE packages RENAME COLUMN updated_at TO updated;

ALTER TABLE package_versions RENAME COLUMN created_at TO created;
ALTER TABLE package_versions RENAME COLUMN updated_at TO updated;
