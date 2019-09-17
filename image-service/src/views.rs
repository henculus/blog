use crate::{error::Error, img::resize, imgur_adapter::upload, multipart_reader::read_buffer};
use log::*;
use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse};
use futures::Future;

pub fn process_image(
    _req: HttpRequest,
    multipart: Multipart,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let width = 10;
    let height = 10;
    let image_name = "image.jpg".to_string();

    read_buffer(multipart)
        .and_then(move |buf| resize(buf.as_slice(), width, height))
        .and_then(|resized| upload(resized, image_name))
        .map(|link| HttpResponse::Ok().json(link))
        .from_err()
}

pub fn index() -> HttpResponse {
    info!("Sending form page");
    let html = r#"
    <html lang="en">
        <head><title>Upload Test</title></head>
        <body>
            <form action="/image" method="post" enctype="multipart/form-data">
                <input type="file" name="file" accept=".jpeg, .jpg"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>
    "#;

    HttpResponse::Ok().body(html)
}