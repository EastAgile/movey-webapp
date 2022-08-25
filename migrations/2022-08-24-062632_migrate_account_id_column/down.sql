-- create account_id column in packages
ALTER TABLE packages ADD account_id INTEGER;

-- move account_id from package_collaborators to packages
DO $$
DECLARE
    temprow RECORD;
BEGIN
    FOR temprow IN
        SELECT package_id, account_id FROM package_collaborators WHERE role = 0
    LOOP
        UPDATE packages SET account_id = temprow.account_id WHERE id = temprow.package_id;
    END LOOP;
END;
$$;

-- create foreign key packages -> accounts
ALTER TABLE packages ADD CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts (id);
