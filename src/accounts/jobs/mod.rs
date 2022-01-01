use jelly::jobs::{JobState, WorkerConfig};

mod verify;
pub use verify::build_context as build_verify_context;
pub use verify::SendVerifyAccountEmail;

mod welcome;
pub use welcome::build_context as build_welcome_context;
pub use welcome::SendWelcomeAccountEmail;

mod reset_password;
pub use reset_password::build_context as build_reset_password_context;
pub use reset_password::{SendPasswordWasResetEmail, SendResetPasswordEmail};

mod odd_registration_attempt;
pub use odd_registration_attempt::build_context as build_odd_registration_attempt_context;
pub use odd_registration_attempt::SendAccountOddRegisterAttemptEmail;

pub fn configure(config: WorkerConfig<JobState>) -> WorkerConfig<JobState> {
    let mut config = config.register::<SendResetPasswordEmail>();
    config = config.register::<SendPasswordWasResetEmail>();
    config = config.register::<SendWelcomeAccountEmail>();
    config = config.register::<SendAccountOddRegisterAttemptEmail>();
    config.register::<SendVerifyAccountEmail>()
}
