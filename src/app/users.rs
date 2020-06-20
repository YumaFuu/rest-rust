use rocket_contrib::json::Json;

use crate::domain::users::*;

#[get("/", format = "application/json")]
pub fn index() -> &'static str {
    // let users = users.all().get_result::<User>(&cn)?;
    "hoge"
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create(user: Json<User>) -> String {
    format!("name: {} email: {}", user.name, user.email)
}
