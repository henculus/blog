#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate serde;

use rocket::Rocket;

mod models;
mod views;
mod tests;

fn create_app() -> Rocket {
    let db_pool = models::db_pool()
        .expect("Cannot connect to database");

    let table_manager = models::TableManager::new(db_pool);

    rocket::ignite()
        .manage(table_manager)
        .register(catchers![views::not_found, views::service_unavailable])
        .mount("/",
               routes![
                   views::new_post,
                   views::get_post,
                   views::get_posts,
                   views::delete_post,
                   views::update_post])
}

fn main() {
    let rocket = create_app();
    rocket.launch();
}