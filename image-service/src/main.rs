use actix_web::{web, App, HttpServer};

mod img;
mod error;
mod imgur_adapter;
mod multipart_reader;

fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().service(web::resource("/")))
        .bind("127.0.0.1:8080")?
        .run()
}
