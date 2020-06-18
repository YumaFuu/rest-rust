#[derive(Serialize, Deserialize)]

pub struct User {
    pub id: u32,
    pub name: &str,
    pub email: &str,
    pub password_digest: &str,
}
