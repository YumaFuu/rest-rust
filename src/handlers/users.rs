#[get("/")]
pub fn index() -> &'static str {
    "All Users"
}
