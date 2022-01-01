use actix_web::{web, HttpRequest};
use background_jobs::Job;
use background_jobs::QueueHandle;

use crate::error::Error;

/// A trait for adding jobs to a background queue.
pub trait JobQueue {
    /// Grabs a QueueHandle and adds the job to the queue.
    fn queue<J: Job + 'static>(&self, job: J) -> Result<(), Error>;
}

impl JobQueue for HttpRequest {
    fn queue<J: Job + 'static>(&self, job: J) -> Result<(), Error> {
        let handle: Option<&web::Data<QueueHandle>> = self.app_data();

        if let Some(handle) = handle {
            handle.queue(job)?;
            return Ok(());
        }

        Err(Error::Generic("QueueHandle unavailable.".to_string()))
    }
}
