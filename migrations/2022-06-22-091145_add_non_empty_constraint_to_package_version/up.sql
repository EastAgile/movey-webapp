ALTER TABLE package_versions
ADD CONSTRAINT non_empty_version CHECK(length(version)>0);
