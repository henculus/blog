use std::iter::Iterator;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{App, Error, HttpResponse, HttpServer, middleware, web};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::{HeaderValue, StatusCode};
use actix_web::http::header::CONTENT_TYPE;
use futures::{Future, Stream};
use futures::future::{FutureResult, ok, err};
use image::{DynamicImage, FilterType};
use image::ImageFormat::JPEG;
use log::*;

fn get_buffer_from_field(field: Field) -> impl Future<Item=Vec<u8>, Error=Error> {
    field
        .fold(vec![], move |mut acc, bytes| {
            info!("Reading buffer from multipart...");
            acc.extend(bytes);
            ok(acc) as FutureResult<Vec<u8>, MultipartError>
        })
        .map_err(|e| {
            error!("Reading multipart failed: {:?}", e);
            ErrorInternalServerError(e)
        })
}

fn get_image_from_buffer(buffer: Vec<u8>) -> impl Future<Item=DynamicImage, Error=Error> {
    info!("Loading image from buffer...");
    match image::load_from_memory(&buffer) {
        Ok(img) => ok(img),
        Err(e) => {
            error!("Image error: {:?}", e);
            err(ErrorInternalServerError(e))
        }
    }
}

fn resize_img(img: DynamicImage) -> impl Future<Item=DynamicImage, Error=Error> {
    web::block(move || {
        info!("Starting resizing image...");
        Ok(img.resize(100, 100, FilterType::Nearest)) as Result<DynamicImage, ()>
    })
        .map_err(|e| {
            error!("Blocking error: {:?}", e);
            ErrorInternalServerError(e)
        })
}

fn load_img_to_buffer(img: DynamicImage) -> impl Future<Item=Vec<u8>, Error=Error> {
    let mut buffer = Vec::new();
    info!("Writing image to buffer...");
    match img.write_to(&mut buffer, JPEG) {
        Ok(_) => ok(buffer),
        Err(e) => {
            error!("Image error: {:?}", e);
            err(ErrorInternalServerError(e))
        }
    }
}

pub fn process(field: Field) -> impl Future<Item=Vec<u8>, Error=Error> {
    get_buffer_from_field(field)
        .and_then(get_image_from_buffer)
        .and_then(resize_img)
        .and_then(load_img_to_buffer)
}

pub fn upload_to_imgur(img: Vec<u8>) -> Result<String, Error> {
    info!("Uploading image to the imgur");
    let client = reqwest::Client::new();
    let image = reqwest::multipart::Part::bytes(img)
        .file_name("img.png")
        .mime_str("image/jpg")
        .map_err(|e| {
            error!("Reqwest multipart error: {}", e);
            ErrorInternalServerError(e)
        })?;
    let form = reqwest::multipart::Form::new()
        .part("image", image);

    let response: serde_json::Value = client
        .post("https://api.imgur.com/3/upload")
        .header("Authorization", "Client-ID cc27cc3925c6140")
        .multipart(form)
        .send()
        .map_err(|e| {
            error!("Request error: {}", e);
            ErrorInternalServerError(e)
        })?
        .json()
        .map_err(|e| {
            error!("Deserialize error: {}", e);
            ErrorInternalServerError(e)
        })?;
    debug!("Request body: {}", response);
    let link = response["data"]["link"].to_string();
    info!("Image uploaded to imgur: {}", link);
    Ok(link)
}

pub fn upload(multipart: Multipart) -> impl Future<Item=HttpResponse, Error=Error> {
    info!("Started image processing...");
    multipart
        .map(|field| process(field).into_stream())
        .flatten()
        .collect()
        .map(|buf| {
            let img: Vec<u8> = buf.iter().cloned().flatten().collect();
            let img_cloned = img.clone();
            web::block(move || {
                let _link = upload_to_imgur(img_cloned).map_err(|e| {
                    error!("Uploading error: {}", e);
                }).unwrap();
                Ok(()) as Result<(), ()>
            }).wait();

            info!("Writing buffer to response...");
            let mut response = HttpResponse::with_body(StatusCode::OK, img.clone().into());
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
            info!("Sending response to user...");
            response
        })
}

fn index() -> HttpResponse {
    info!("Sending form page");
    let html = r#"
    <html lang="en">
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file" accept=".jpeg"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>
    "#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
//    std::env::set_var(
//        "RUST_LOG",
//        "image_service=debug,actix_server=info,actix_web=info",
//    );
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
