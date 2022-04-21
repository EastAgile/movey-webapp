-- Seed data for packages
INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand', 100000, 'Random number generators and other randomness functionality.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('random_derive', 1300, 'Procedurally defined macro for automatically deriving rand::Rand for structs and enums', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('faker_rand', 2000, 'Fake data generators for lorem ipsum, names, emails, and more', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_derive2', 311, 'Generate customizable random types with the rand crate', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('diesel', 10, 'A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL', 'https://github.com/diesel-rs/diesel');

-- Seed data for versions
-- Assuming diesel package id is always 2, if broken just comment out this part
INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (2, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (2, '1.1.0', 'Read me plz! Updated', 'Apache', '01c84198819310620f2417413c3c800df82xxxx', 4 ,200);
