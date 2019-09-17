use crate::{error::Error, img::resize, imgur_adapter::upload, multipart_reader::read_buffer};
use actix_multipart::Multipart;
use serde::Deserialize;
use actix_web::{web::Query, HttpRequest, HttpResponse};
use futures::Future;
use log::*;

#[derive(Debug, Deserialize)]
pub struct ProcessImageInfo {
    #[serde(default = "default_side")]
    width: u32,
    #[serde(default = "default_side")]
    height: u32,
}

fn default_side() -> u32 {
    300
}

pub fn process_image(
    Query(info): Query<ProcessImageInfo>,
    req: HttpRequest,
    multipart: Multipart,
) -> impl Future<Item=HttpResponse, Error=Error> {
    debug!("Request to resize image: {:?}", req);
    let width = info.width;
    let height = info.height;
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
            <form action="/image?width=100" method="post" enctype="multipart/form-data">
                <input type="file" name="file" accept=".jpeg, .jpg"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>
    "#;

    HttpResponse::Ok().body(html)
}
