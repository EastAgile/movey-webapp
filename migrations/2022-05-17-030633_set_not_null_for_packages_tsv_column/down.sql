-- This file should undo anything in `up.sql`

ALTER TABLE packages ALTER COLUMN tsv DROP NOT NULL;
