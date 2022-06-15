-- This file should undo anything in `up.sql`

ALTER TABLE accounts
DROP COLUMN github_id;
