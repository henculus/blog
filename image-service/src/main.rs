use std::iter::Iterator;

use actix_multipart::{Field, Multipart};
use actix_web::{App, Error, HttpResponse, HttpServer, middleware, web};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::{HeaderValue, StatusCode};
use actix_web::http::header::CONTENT_TYPE;
use futures::{Future, Stream};
use futures::future::ok;
use futures::stream::StreamFuture;
use image::{DynamicImage, FilterType};
use image::ImageFormat::JPEG;
use log::*;

pub fn get_field_from_multipart(multipart: StreamFuture<Multipart>) -> impl Future<Item=Field, Error=Error> {
    info!("Getting field from multipart...");
    multipart
        .map(|(f, _)| f.unwrap())
        .map_err(|(e, _)| e)
        .from_err()
}

pub fn get_img_from_field(field: Field) -> impl Future<Item=DynamicImage, Error=Error> {
    info!("Getting image from the field...");
    field
        .map_err(ErrorInternalServerError)
        .collect()
        .map_err(ErrorInternalServerError)
        .map(|buf| {
            buf.iter().fold(vec![], |mut a, b| {
                a.extend(b);
                a
            })
        }
        )
        .and_then(|buf|
            ok(image::load_from_memory(&buf).unwrap())
        )
}

pub fn resize_img(img: DynamicImage) -> impl Future<Item=DynamicImage, Error=Error> {
    info!("Resizing image...");
    ok(img.resize(100, 100, FilterType::Nearest))
}

pub fn to_buffer(img: DynamicImage) -> impl Future<Item=Vec<u8>, Error=Error> {
    info!("Writing image to the buffer...");
    let mut buffer = Vec::new();
    img.write_to(&mut buffer, JPEG).unwrap();
    ok(buffer)
}

pub fn upload(multipart: Multipart) -> impl Future<Item=HttpResponse, Error=Error> {
    info!("Started image processing...");
    get_field_from_multipart(multipart.into_future())
        .and_then(|field| get_img_from_field(field))
        .and_then(|img| resize_img(img))
        .and_then(|img| to_buffer(img))
        .and_then(|buffer| {
            let mut response = HttpResponse::with_body(StatusCode::OK, buffer.into());
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
            response
        })
}

fn index() -> HttpResponse {
    info!("Sending form page");
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file" accept=".png"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
//    std::env::set_var("RUST_LOG", "image_service=debug,actix_server=info,actix_web=info");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to_async(upload)),
        )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
