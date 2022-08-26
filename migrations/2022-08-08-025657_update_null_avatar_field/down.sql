-- This file should undo anything in `up.sql`

UPDATE accounts 
SET avatar = NULL
WHERE avatar is not NULL and github_login is NULL;