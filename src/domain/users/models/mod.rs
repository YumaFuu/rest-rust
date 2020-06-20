use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password_digest: String,
}
