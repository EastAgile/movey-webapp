-- Your SQL goes here

ALTER TABLE package_versions
    ADD total_files integer;

ALTER TABLE package_versions
    ADD total_size integer;