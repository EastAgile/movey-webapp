use serde::Serialize;

#[derive(Serialize)]
pub struct Collaborator {
    pub role: Role,
    pub email: String,
    pub id: i32,
}

#[derive(Serialize)]
pub enum Role {
    OWNER,
    COLLABORATOR,
    PENDING,
    EXTERNAL,
}
