use crate::views::{index, process_image};
use actix_web::{web, App, HttpServer};

mod error;
mod img;
mod imgur_adapter;
mod multipart_reader;
mod views;

fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(
            web::resource("/image")
                .route(web::post().to_async(process_image))
                .route(web::get().to(index)),
        )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
