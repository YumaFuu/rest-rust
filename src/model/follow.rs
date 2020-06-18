#[derive(Serialize, Deserialize)]

pub struct Follow {
    id: u32,
    user_id: u32,
    follow_user_id: u32,
}
