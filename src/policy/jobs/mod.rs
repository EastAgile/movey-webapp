use jelly::jobs::{JobState, WorkerConfig};

mod contact;
pub use contact::*;

pub fn configure(config: WorkerConfig<JobState>) -> WorkerConfig<JobState> {
    let config = config.register::<SendContactRequestEmail>();
    config.register::<SendContactEmail>()
}
