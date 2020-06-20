use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use schema::{users, posts, follows}

// pub fn establish_connection() -> MysqlConnection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: u32,
    pub name: &str,
    pub email: &str,
    pub password_digest: &str,
}

#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: u32,
    pub user_id: u32,
    pub text: %str,
}

