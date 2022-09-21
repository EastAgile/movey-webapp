-- Seed data for packages
INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand', 100000, 'Random number generators and other randomness functionality.', 'https://github.com/rust-random/rand', 'rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('random_derive', 1300, 'Procedurally defined macro for automatically deriving rand::Rand for structs and enums', 'https://github.com/rust-random/rand', 'random-derive');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('faker_rand', 20000, 'Fake data generators for lorem ipsum, names, emails, and more', 'https://github.com/rust-random/rand', 'faker-rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_derive2', 3110, 'Generate customizable random types with the rand crate', 'https://github.com/rust-random/rand', 'rand-derive2');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('diesel', 5000, 'A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL', 'https://github.com/diesel-rs/diesel', 'diesel');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('ndarray-rand', 1234, 'Constructors for randomized arrays. `rand` integration for `ndarray`.', 'https://github.com/rust-random/rand', 'ndarray-rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_core', 5432, 'Core random number generator traits and tools for implementation.', 'https://github.com/rust-random/rand', 'rand-core');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_mt', 6121, 'Reference Mersenne Twister random number generators.', 'https://github.com/rust-random/rand', 'rand-mt');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('pcg_rand', 6416, 'An implementation of the PCG family of random number generators in pure Rust', 'https://github.com/rust-random/rand', 'pcg-rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_macros', 25656, '`#[derive]`-like functionality for the `rand::Rand` trait.', 'https://github.com/rust-random/rand', 'rand-macros');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('quad-rand', 56489, 'Pseudo random implementation with std atomics.', 'https://github.com/rust-random/rand', 'quad-rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand-bytes', 9846, 'A simple tool to generate cryptographically secure random bytes using a cryptographic pseudo-random number generator.', 'https://github.com/rust-random/rand', 'rand-bytes');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand-facade', 789, 'A global mutex-based random facade for no_std compatible libraries that require an initialised random number generator.', 'https://github.com/rust-random/rand', 'rand-facade');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_distr', 56515, 'Sampling from random number distributions', 'https://github.com/rust-random/rand', 'rand-distr');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_jitter', 516564, 'Random number generator based on timing jitter', 'https://github.com/rust-random/rand', 'rand-jitter');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_os', 89521, 'OS backed Random Number Generator', 'https://github.com/rust-random/rand', 'rand-os');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_xoshiro', 51694, 'Xoshiro, xoroshiro and splitmix64 random number generators', 'https://github.com/rust-random/rand', 'rand-xoshiro');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_hc', 849145, 'HC128 random number generator', 'https://github.com/rust-random/rand', 'rand_hc');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_isaac', 98414, 'ISAAC random number generator', 'https://github.com/rust-random/rand', 'rand-isaac');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_xorshift', 123456, 'Xorshift random number generator', 'https://github.com/rust-random/rand', 'rand-xorshift');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('getrandom', 515651, 'A small cross-platform library for retrieving random data from system source', 'https://github.com/rust-random/rand', 'getrandom');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('fastrand', 123456, 'A simple and fast random number generator', 'https://github.com/rust-random/rand', 'fastrand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand_chacha', 16565, 'ChaCha random number generator', 'https://github.com/rust-random/rand', 'rand-chacha');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('cap-rand', 94748, 'Capability-based random number generators', 'https://github.com/rust-random/rand', 'cap-rand');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand04', 8462, 'Re-export of rand 0.4, so it can be used together with a later version of rand.', 'https://github.com/rust-random/rand', 'rand04');

INSERT INTO packages(name, total_downloads_count, description, repository_url, slug)
    VALUES ('rand04_compat', 15995, 'Wrappers for compatibility with rand 0.4.', 'https://github.com/rust-random/rand', 'rand04-compat');
