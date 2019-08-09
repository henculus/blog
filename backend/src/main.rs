#![feature(
proc_macro_hygiene,
decl_macro,
type_alias_enum_variants,
type_ascription
)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;

use std::io;
use std::path::{Path, PathBuf};

use rocket::{Request, Response, Rocket};
use rocket::http::{Method, Status};
use rocket::response::{NamedFile, Responder};
use rocket::request::{FromRequest, Outcome};
use rocket_contrib::helmet::{Hsts, SpaceHelmet};
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use time::Duration;

use crate::error::Error;

mod error;
mod post;
mod posts_view;
mod schema;
mod user;
mod users_view;

#[cfg(test)]
mod tests;

type Id = i32;
type ViewResult<T> = std::result::Result<Json<T>, Error>;

#[database("blog")]
pub struct Database(diesel::PgConnection);

fn configure_cors() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(
            &[
                "https://www.lupusanay.me",
                "https://lupusanay.me",
                "http://localhost:8080",
                "https://localhost:8080",
                "http://0.0.0.0:8080",
                "https://0.0.0.0:8080",
                "http://localhost:8000",
                "https://localhost:8000",
                "http://0.0.0.0:8000",
                "https://0.0.0.0:8000"
            ],
        );

    let allowed_methods = vec![Method::Get, Method::Post, Method::Put, Method::Delete]
        .into_iter()
        .map(From::from)
        .collect();

    let allowed_headers = AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]);

    let allow_credentials = true;

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials,
        ..Default::default()
    }
        .to_cors()
        .unwrap()
}

fn create_app() -> Rocket {
    let cors = configure_cors();

    rocket::ignite()
        .mount(
            "/",
            routes![
                posts_view::new_post,
                posts_view::get_users_post,
                posts_view::get_post,
                posts_view::get_posts,
                posts_view::delete_post,
                posts_view::update_post,
                users_view::new_user,
                users_view::login,
                users_view::get_session_info,
                users_view::get_users,
                users_view::logout,
                users_view::delete_user
            ],
        )
        .attach(cors)
        .attach(SpaceHelmet::default().enable(Hsts::IncludeSubDomains(Duration::days(30))))
        .attach(Database::fairing())
}

fn main() {
    let rocket = create_app();
    rocket.launch();
}
