#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;
extern crate serde_derive;

mod handlers;

fn main() {
    rocket::ignite()
        .mount("/", routes![handlers::posts::index])
        .mount(
            "/users",
            routes![handlers::users::index, handlers::users::create],
        )
        .launch();
}
