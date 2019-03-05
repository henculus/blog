#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

extern crate serde;
extern crate dotenv;

mod models;
mod views;

fn main() {
    rocket::ignite()
        .register(catchers![views::not_found])
        .mount("/",
               routes![views::new_post, views::get_post, views::get_posts])
        .launch();
}
