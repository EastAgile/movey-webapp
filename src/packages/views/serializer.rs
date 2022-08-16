use serde::Serialize;

#[derive(Serialize)]
pub struct Collaborator {
    pub role: Role,
    pub email: String,
}

#[derive(Serialize)]
pub enum Role {
    OWNER,
    COLLABORATOR,
    PENDING,
    USER,
}