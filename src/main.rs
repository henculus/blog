#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;

extern crate serde;
extern crate dotenv;
extern crate r2d2;

use crate::models::db_pool;


mod models;
mod views;

fn main() {
    let db_pool = db_pool().expect("Cannot connect to database");

    rocket::ignite()
        .manage(db_pool)
        .register(catchers![views::not_found])
        .mount("/",
               routes![
                   views::new_post,
                   views::get_post,
                   views::get_posts,
                   views::delete_post,
                   views::update_post])
        .launch();
}
