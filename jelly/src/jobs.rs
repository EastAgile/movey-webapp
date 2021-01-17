//! This module contains types used in Job registration and handling.

use sqlx::postgres::PgPool;

pub use background_jobs::{Job, WorkerConfig};

pub const DEFAULT_QUEUE: &'static str = "default";

/// This type can be used to indicate what environment a job is running in,
/// as well as gaining access to a database connection.
#[derive(Clone)]
pub struct JobState {
    pub name: String,
    pub pool: PgPool
}

impl JobState {
    /// Creates a new `JobState` object.
    pub fn new(name: &str, pool: PgPool) -> Self {
        JobState {
            name: name.to_owned(),
            pool: pool
        }
    }
}

