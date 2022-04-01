ALTER TABLE accounts
ADD CONSTRAINT non_empty CHECK(length(email)>0);
