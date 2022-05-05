-- Seed data for packages
INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand', 100000, 'Random number generators and other randomness functionality.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('random_derive', 1300, 'Procedurally defined macro for automatically deriving rand::Rand for structs and enums', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('faker_rand', 20000, 'Fake data generators for lorem ipsum, names, emails, and more', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_derive2', 3110, 'Generate customizable random types with the rand crate', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('diesel', 5000, 'A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL', 'https://github.com/diesel-rs/diesel');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('ndarray-rand', 1234, 'Constructors for randomized arrays. `rand` integration for `ndarray`.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_core', 5432, 'Core random number generator traits and tools for implementation.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_mt', 6121, 'Reference Mersenne Twister random number generators.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('pcg_rand', 6416, 'An implementation of the PCG family of random number generators in pure Rust', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_macros', 25656, '`#[derive]`-like functionality for the `rand::Rand` trait.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('quad-rand', 56489, 'Pseudo random implementation with std atomics.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand-bytes', 9846, 'A simple tool to generate cryptographically secure random bytes using a cryptographic pseudo-random number generator.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand-facade', 789, 'A global mutex-based random facade for no_std compatible libraries that require an initialised random number generator.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_distr', 56515, 'Sampling from random number distributions', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_jitter', 516564, 'Random number generator based on timing jitter', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_os', 89521, 'OS backed Random Number Generator', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_xoshiro', 51694, 'Xoshiro, xoroshiro and splitmix64 random number generators', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_hc', 849145, 'HC128 random number generator', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_isaac', 98414, 'ISAAC random number generator', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_xorshift', 123456, 'Xorshift random number generator', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('getrandom', 515651, 'A small cross-platform library for retrieving random data from system source', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('fastrand', 123456, 'A simple and fast random number generator', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand_chacha', 16565, 'ChaCha random number generator', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('cap-rand', 94748, 'Capability-based random number generators', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand04', 8462, 'Re-export of rand 0.4, so it can be used together with a later version of rand.', 'https://github.com/rust-random/rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url)
    VALUES ('rand04_compat', 15995, 'Wrappers for compatibility with rand 0.4.', 'https://github.com/rust-random/rand');

-- Seed data for versionst
INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (1, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (2, '1.1.0', 'Read me plz! Updated', 'Apache', '01c84198819310620f2417413c3c800df82xxxx', 4 ,200);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (3, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (4, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (5, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (6, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (7, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (8, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (9, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (10, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (11, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (12, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (13, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (14, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (15, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (16, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (17, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (18, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (19, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (20, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (21, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (22, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (23, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (24, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (25, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

INSERT INTO package_versions(package_id, version, readme_content, license, rev, total_files,total_size)
    VALUES (26, '1.0.0', 'Read me plz!', 'MIT', '01c84198819310620f2417413c3c800df8292ae5', 2, 100);

    
