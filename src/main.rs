#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

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
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod posts_view;
mod users_view;
mod user;
mod post;
mod schema;

type Id = i32;
type ViewResult<T> = std::result::Result<Json<T>, Error>;
type ModelResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnimplementedError,
    WrongAuthType,
    NoAuthHeader,
    TokenError,
    WrongPassword,
}

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Error::UnimplementedError
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        Error::UnimplementedError
    }
}

#[database("blog")]
pub struct Database(diesel::PgConnection);

fn create_app() -> Rocket {
    let allowed_origins =
        AllowedOrigins::some(&["http://www.lupusanay.me"], &["http://localhost:8080"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .unwrap();

    rocket::ignite()
        .mount(
            "/api",
            routes![
                posts_view::new_post,
                posts_view::get_post,
                posts_view::get_posts,
                posts_view::delete_post,
                posts_view::update_post,
                users_view::new_user,
                users_view::login,
            ],
        )
        .attach(cors)
        .attach(Database::fairing())
}

fn main() {
    let rocket = create_app();
    rocket.launch();
}
