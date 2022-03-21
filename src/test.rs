use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::env;
use jelly::database;

use diesel::prelude::*;
use diesel::pg::PgConnection;

#[cfg(test)]
use dotenv::dotenv;

lazy_static! {
   static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

lazy_static! {
    pub static ref DB_POOL: jelly::DieselPgPool = database::init_database();
}

#[cfg(test)]
pub fn init() {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        dotenv().ok();
        *initiated = true;
    }
}

embed_migrations!("migrations/");

const TEST_DB_NAME: &str = "movey_app_test";

pub struct DatabaseTestContext {

}

#[cfg(test)]
impl DatabaseTestContext {
    pub fn new() -> Self {
        let database_base_url = env::var("DATABASE_URL_TEST_BASE").expect("DATABASE_URL_TEST_BASE must be set");
        let conn = PgConnection::establish(&database_base_url).expect("Cannot connect to base database.");

        let query = diesel::sql_query(format!("CREATE DATABASE {}", TEST_DB_NAME).as_str());
        query
            .execute(&conn)
            .expect(format!("Could not create database {}", TEST_DB_NAME).as_str());

        let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
        let conn = PgConnection::establish(&database_url).expect("Cannot connect to test database.");
        embedded_migrations::run(&conn).unwrap();

        Self {}
    }
}

impl Drop for DatabaseTestContext {
    fn drop(&mut self) {
        let database_base_url = env::var("DATABASE_URL_TEST_BASE")
        .expect("DATABASE_URL_TEST_BASE must be set");
        let conn =
            PgConnection::establish(&database_base_url).expect("Cannot connect to test database.");

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = '{}';",
            TEST_DB_NAME
        );

        diesel::sql_query(disconnect_users.as_str())
            .execute(&conn)
            .unwrap();


        let query = diesel::sql_query(format!("DROP DATABASE {}", TEST_DB_NAME).as_str());
        query
            .execute(&conn)
            .expect(&format!("Couldn't drop database {}", TEST_DB_NAME));
    }
}
