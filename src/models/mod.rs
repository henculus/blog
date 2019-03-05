use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod post;
pub mod schema;
pub mod errors;

pub type Id = i32;

pub fn connection() -> PgConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}