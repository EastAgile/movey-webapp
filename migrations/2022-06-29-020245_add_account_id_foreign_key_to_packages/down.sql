-- This file should undo anything in `up.sql`

ALTER TABLE packages
DROP CONSTRAINT fk_account_id;
