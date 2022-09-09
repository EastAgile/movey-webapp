use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CollaboratorJson {
    pub user: String,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct InvitationResponse {
    pub package_id: i32,
    pub accepted: bool,
}
