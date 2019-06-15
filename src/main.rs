#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants, type_ascription)]

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

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket::Rocket;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

use crate::error::Error;

mod posts_view;
mod users_view;
mod user;
mod post;
mod schema;
mod error;

#[cfg(test)]
mod tests;

type Id = i32;
type ViewResult<T> = std::result::Result<Json<T>, Error>;

#[database("blog")]
pub struct Database(diesel::PgConnection);

#[get("/", rank = 10)]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/dist/index.html")
}

// TODO: Handle forwarding to this request from /api routes, may be with request guard
#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    match NamedFile::open(Path::new("static/dist/").join(file)) {
        Ok(file) => Some(file),
        Err(_) => NamedFile::open("static/dist/index.html").ok(),
    }
}

fn configure_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some(
        &["http://www.lupusanay.me"],
        &["http://localhost:8080"],
    );

    let allowed_methods = vec![
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete
    ]
        .into_iter()
        .map(From::from)
        .collect();

    let allowed_headers = AllowedHeaders::some(
        &[
            "Authorization",
            "Accept",
            "Content-Type"
        ]
    );

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
            "/api",
            routes![
                posts_view::new_post,
                posts_view::get_users_post,
                posts_view::get_post,
                posts_view::get_posts,
                posts_view::delete_post,
                posts_view::update_post,
                posts_view::publish_post,
                users_view::new_user,
                users_view::login,
            ],
        )
        .mount(
            "/",
            routes![
                index,
                files,
            ],
        )
        .attach(cors)
        .attach(Database::fairing())
}

fn main() {
    let rocket = create_app();
    rocket.launch();
}
