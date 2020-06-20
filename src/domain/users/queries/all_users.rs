use crate::domain::users::User;

pub fn all(cn: &MysqlConnection) -> Vec<User> {
    users::table.order(users::id).load::<User>(cn).unwrap()
}
