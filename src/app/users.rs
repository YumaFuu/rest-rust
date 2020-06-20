use self::schema::{follows, posts, users};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[get("/", format = "application/json")]
pub fn index() -> &'static str {
    let users = users.all().get_result::<User>(&cn)?;
    "hoge"
}

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
    password_confirm: String,
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(user: Json<User>) -> String {
    format!("name: {} email: {}", user.name, user.email)
}
