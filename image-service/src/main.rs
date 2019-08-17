use actix_multipart::{Field, Multipart};
use actix_web::{App, Error, HttpResponse, HttpServer, middleware, web};
use actix_web::http::{HeaderValue, StatusCode};
use actix_web::http::header::CONTENT_TYPE;
use futures::{Future, Stream};
use image::{DynamicImage, FilterType};
use image::ImageFormat::JPEG;
use futures::stream::StreamFuture;

pub fn get_field_from_multipart(multipart: StreamFuture<Multipart>) -> impl Future<Item=Field, Error=Error> {
    multipart
        .map(|(f, _)| f.unwrap())
        .map_err(|(e, _)| e)
        .from_err()
}

pub fn get_img_from_field(field: Field) -> impl Future<Item=DynamicImage, Error=Error> {
    field
        .concat2()
        .from_err()
        .and_then(move |bytes| web::block(move || image::load_from_memory(&bytes)).from_err())
}

pub fn resize_img(img: DynamicImage) -> impl Future<Item=DynamicImage, Error=Error> {
    web::block(move || Ok(img.resize(100, 100, FilterType::Nearest)) as Result<_, ()>).from_err()
}

pub fn to_buffer(img: DynamicImage) -> impl Future<Item=Vec<u8>, Error=Error> {
    web::block(move || {
        let mut buffer = Vec::new();
        img.write_to(&mut buffer, JPEG).unwrap();
        Ok(buffer) as Result<_, ()>
    })
        .from_err()
}

pub fn to_response(buffer: Vec<u8>) -> HttpResponse {
    let mut response = HttpResponse::with_body(StatusCode::OK, buffer.into());
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
    response
}

pub fn upload(multipart: Multipart) -> impl Future<Item=HttpResponse, Error=Error> {
    get_field_from_multipart(multipart.into_future())
        .and_then(get_img_from_field)
        .and_then(resize_img)
        .and_then(to_buffer)
        .and_then(to_response)
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
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
