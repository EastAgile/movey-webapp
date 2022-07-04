-- Your SQL goes here

ALTER TABLE packages
ADD CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts (id);
