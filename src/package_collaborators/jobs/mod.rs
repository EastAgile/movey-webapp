use jelly::jobs::{JobState, WorkerConfig};

use {
    invite_collaborator::{SendCollaboratorInvitationEmail, SendRegisterToCollabEmail},
    transfer_ownership::SendOwnershipTransferEmail,
};

pub mod invite_collaborator;
pub mod transfer_ownership;

pub fn configure(config: WorkerConfig<JobState>) -> WorkerConfig<JobState> {
    let mut config = config.register::<SendCollaboratorInvitationEmail>();
    config = config.register::<SendRegisterToCollabEmail>();
    config.register::<SendOwnershipTransferEmail>()
}
