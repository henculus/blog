#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;

use rocket::http::Method;
use rocket::Rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod models;
mod views;

#[database("blog")]
pub struct DBConn(diesel::PgConnection);

fn create_app() -> Rocket {
    let allowed_origins =
        AllowedOrigins::some(&["http://www.lupusanay.me"], &["http://localhost:8080"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .unwrap();

    rocket::ignite()
        .register(catchers![
            views::not_found,
            views::service_unavailable,
            views::bad_request,
            views::unprocessable_entity
        ])
        .mount(
            "/api",
            routes![
                views::posts::new_post,
                views::posts::get_post,
                views::posts::get_posts,
                views::posts::delete_post,
                views::posts::update_post,
                views::users::new_user,
                views::users::login
            ],
        )
        .mount("/", routes![views::index, views::files,])
        .attach(cors)
        .attach(DBConn::fairing())
}

fn main() {
    let rocket = create_app();
    rocket.launch();
}
