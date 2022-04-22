-- This file should undo anything in `up.sql`

ALTER TABLE package_versions
    DROP COLUMN total_files;


ALTER TABLE package_versions
    DROP COLUMN total_size;
