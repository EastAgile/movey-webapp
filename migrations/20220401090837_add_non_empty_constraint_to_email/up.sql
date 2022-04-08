ALTER TABLE accounts
ADD CONSTRAINT non_empty_email CHECK(length(email)>0);
