use std::env;
use diesel::r2d2::{Pool, ConnectionManager};
use crate::DieselPgPool;

pub fn init_database() -> DieselPgPool {
    let db_uri = if cfg!(any(test, feature = "test")) {
        env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST not set!")
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL not set!")
    };

    let pool_size = env::var("DATABASE_POOL_SIZE")
        .unwrap_or_else(|_| "15".to_string())
        .parse()
        .expect("DATABASE_POOL_SIZE must be a number");
    let manager = ConnectionManager::new(db_uri);
    let pool = Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Unable to connect to database!");
    pool
}
