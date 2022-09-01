-- move account_id from packages to package_collaborators
DO $$
DECLARE
    temprow RECORD;
BEGIN
    FOR temprow IN
        SELECT id, account_id FROM packages WHERE account_id IS NOT NULL
    LOOP
        IF NOT EXISTS(
            SELECT 1
            FROM package_collaborators
            WHERE package_id = temprow.id AND account_id = temprow.account_id
        ) THEN
            INSERT INTO package_collaborators VALUES (temprow.id, temprow.account_id, 0, temprow.account_id, NOW());
        END IF;
    END LOOP;
END;
$$;

-- delete account_id column from packages
ALTER TABLE packages DROP CONSTRAINT fk_account_id;
ALTER TABLE packages DROP COLUMN account_id;
