use serde::Serialize;

#[derive(Serialize)]
pub struct Invitation {
    pub status: InvitationStatus,
    pub email: String,
}

#[derive(Serialize)]
pub enum InvitationStatus {
    ACCEPTED,
    PENDING,
    ANONYMOUS,
}