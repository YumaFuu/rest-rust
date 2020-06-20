// pub mod follows;
pub mod posts;
pub mod users;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
// use schema::{follows, posts, users};
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
