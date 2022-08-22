use std::hash::{Hash, Hasher};
use serde::Serialize;

#[derive(Serialize, Eq)]
pub struct SerializableInvitation {
    pub status: Status,
    pub email: String,
}

impl PartialEq for SerializableInvitation {
    fn eq(&self, other: &SerializableInvitation) -> bool {
        self.email == other.email
    }
}

impl Hash for SerializableInvitation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

#[derive(Serialize, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Status {
    Owner,
    PendingOwner,
    Collaborator,
    PendingCollaborator,
    External,
}
