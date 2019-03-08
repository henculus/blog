#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;

extern crate serde;
extern crate dotenv;
extern crate r2d2;

mod models;
mod views;

fn main() {
    let db_pool = models::db_pool().expect("Cannot connect to database");
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
        .launch();
}
