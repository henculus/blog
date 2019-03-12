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
mod tests;
mod views;

fn create_app() -> Rocket {
    let db_pool = models::db_pool().expect("Cannot connect to database");

    let table_manager = models::TableManager::new(db_pool);

    rocket::ignite()
        .manage(table_manager)
        .register(
            catchers![
                views::not_found,
                views::service_unavailable,
                views::bad_request
            ])
        .mount(
            "/",
            routes![
                views::posts::new_post,
                views::posts::get_post,
                views::posts::get_posts,
                views::posts::delete_post,
                views::posts::update_post
            ],
        )
}

fn main() {
    let rocket = create_app();
    rocket.launch();
}
