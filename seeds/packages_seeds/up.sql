-- Seed data for packages
INSERT INTO packages(name, description, repository_url)
    VALUES ('rand', 'Random number generators and other randomness functionality.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, description, repository_url)
    VALUES ('diesel', 'A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL', 'https://github.com/diesel-rs/diesel');

-- Seed data for versions
-- Assuming diesel package id is always 2, if broken just comment out this part
INSERT INTO package_versions(package_id, version, readme_content, license, rev)
    VALUES (2, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5');

INSERT INTO package_versions(package_id, version, readme_content, license, rev)
    VALUES (2, '1.1.0', 'Read me plz! Updated', 'Apache', '01c84198819310620f2417413c3c800df82xxxx');
