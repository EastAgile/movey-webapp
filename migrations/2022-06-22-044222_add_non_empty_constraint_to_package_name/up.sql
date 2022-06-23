ALTER TABLE packages
ADD CONSTRAINT non_empty_name CHECK(length(name)>0);
