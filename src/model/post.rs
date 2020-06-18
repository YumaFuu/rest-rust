#[derive(Serialize, Deserialize)]

pub struct Post {
    id: u32,
    user_id: u32,
    text: %str,
}
