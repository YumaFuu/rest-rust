#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;
extern crate serde_derive;

mod app;

fn main() {
    rocket::ignite()
        .mount("/", routes![app::posts::index])
        .mount("/users", routes![app::users::index, app::users::create])
        .launch();
}
