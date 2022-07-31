use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct AddCollaboratorJson {
    pub user: String,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct InvitationResponse {
    pub package_id: i32,
    pub accepted: bool,
}
