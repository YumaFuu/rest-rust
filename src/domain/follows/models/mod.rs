#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Follow {
    pub id: u32,
    pub user_id: u32,
    pub follow_user_id: u32,
}
