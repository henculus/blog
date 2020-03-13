use actix_web::{App, HttpServer};
use api::*;
use std::io;

mod api;
mod data;
mod database;
mod error;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(get_post))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
