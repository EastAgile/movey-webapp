use actix_web::HttpRequest;

use crate::{DieselPgConnection, DieselPgPool};
use crate::error::Error;

/// A basic trait to extract a Database Pool instance for use in views and the like.
/// The impetus for this is that Extractors are visually hard to scan, and this does
/// the same thing - and avoids us having to double-Arc our internal PgConnection instances.
pub trait DatabasePool {
    /// Returns a PgConnection reference that can be used for database operations.
    /// Will return an error if, for some reason, it's unable to unwrap and get
    /// the reference.
    fn db_pool(&self) -> Result<&DieselPgPool, Error>;
    fn db_connection(&self) -> Result<DieselPgConnection, Error>;
}

impl DatabasePool for HttpRequest {
    /// Returns a database pool object.
    fn db_pool(&self) -> Result<&DieselPgPool, Error> {
        if let Some(pool) = self.app_data::<DieselPgPool>() {
            return Ok(&pool);
        }

        Err(Error::Generic(
            "Unable to retrieve Database Pool.".to_string(),
        ))
    }

    fn db_connection(&self) -> Result<DieselPgConnection, Error> {
        let pool = self.db_pool()?;
        pool.get().map_err(|_e| Error::Generic(
            "Unable to retrieve Database Connection.".to_string()
        ))
    }
}
