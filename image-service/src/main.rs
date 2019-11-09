use crate::views::{index, process_image};
use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;

mod error;
mod img;
mod imgur_adapter;
mod multipart_reader;
mod views;

fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("https://www.lupusanay.me")
                    .allowed_origin("https://lupusanay.me")
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("https://localhost:8080")
                    .allowed_origin("http://0.0.0.0:8080")
                    .allowed_origin("https://0.0.0.0:8080")
                    .allowed_origin("http://localhost:8000")
                    .allowed_origin("https://localhost:8000")
                    .allowed_origin("http://0.0.0.0:8000")
                    .allowed_origin("https://0.0.0.0:8000")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE))
            .service(
                web::resource("/image")
                    .route(web::post().to_async(process_image))
                    .route(web::get().to(index)),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
