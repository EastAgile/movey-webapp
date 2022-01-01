//! This module contains types used in Job registration and handling.

use sqlx::postgres::PgPool;
use std::sync::{Arc, RwLock};
use tera::Tera;

pub use background_jobs::{Job, WorkerConfig};

pub const DEFAULT_QUEUE: &'static str = "default";

/// This type can be used to indicate what environment a job is running in,
/// as well as gaining access to a database connection and to template engine.
#[derive(Clone)]
pub struct JobState {
    pub name: String,
    pub pool: PgPool,
    pub templates: Arc<RwLock<Tera>>,
}

impl JobState {
    /// Creates a new `JobState` object.
    pub fn new(name: &str, pool: PgPool, templates: Arc<RwLock<Tera>>) -> Self {
        JobState {
            name: name.to_owned(),
            pool,
            templates,
        }
    }
}
