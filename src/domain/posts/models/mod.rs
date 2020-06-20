#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: u32,
    pub user_id: u32,
    pub text: %str,
}

