use actix_web::HttpRequest;
use sqlx::postgres::PgPool;

use crate::error::Error;

/// A basic trait to extract a Database Pool instance for use in views and the like.
/// The impetus for this is that Extractors are visually hard to scan, and this does
/// the same thing - and avoids us having to double-Arc our internal PgPool instances.
pub trait DatabasePool {
    /// Returns a PgPool reference that can be used for database operations.
    /// Will return an error if, for some reason, it's unable to unwrap and get
    /// the reference.
    fn db_pool(&self) -> Result<&PgPool, Error>;
}

impl DatabasePool for HttpRequest {
    /// Returns a database pool object.
    fn db_pool(&self) -> Result<&PgPool, Error> {
        if let Some(pool) = self.app_data::<PgPool>() {
            return Ok(&pool);
        }

        Err(Error::Generic(
            "Unable to retrieve Database Pool.".to_string(),
        ))
    }
}
