use crate::{error::Error, img::resize, imgur_adapter::upload, multipart_reader::read_buffer};
use actix_multipart::Multipart;
use actix_web::{web::Query, HttpResponse};
use futures::{future::ok, future::Either, Future};
use log::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct Links {
    high_resolution: String,
    low_resolution: String,
}

pub fn process_image(
    info: Query<ProcessImageInfo>,
    multipart: Multipart,
) -> impl Future<Item=HttpResponse, Error=Error> {
    info!("Request to resize image");
    let width = info.width;
    let height = info.height;

    read_buffer(multipart)
        .and_then(move |buffer| {
            resize(buffer.clone(), width, height)
                .select2(upload(buffer, "high_res.jpg".into()))
                .map_err(|either| either.split().0)
        })
        .and_then(|either| match either {
            Either::A((buffer, upload_future)) => {
                info!("Resizing completed earlier than uploading");
                info!("Continuing uploading and starting uploading of resized image");

                Either::A(upload_future.join(upload(buffer, "low_rez.jpg".into())))
            }
            Either::B((high_resolution_link, resize_future)) => {
                info!("Uploading completed earlier than resizing");
                info!("Continuing resizing...");

                Either::B(
                    resize_future
                        .and_then(|buf| {
                            info!("Uploading low resolution image");
                            upload(buf, "low_res.jpg".into())
                        })
                        .join(ok(high_resolution_link)),
                )
            }
        })
        .from_err()
        .and_then(|(high_resolution, low_resolution)| {
            info!("Sending response to the client: high {}; low {}", high_resolution, low_resolution);
            HttpResponse::Ok().json(Links {
                high_resolution,
                low_resolution,
            })
        })
        .from_err()
}

pub fn index() -> HttpResponse {
    info!("Sending form page");
    let html = r#"
    <html lang="en">
        <head><title>Upload test</title></head>
        <body>
            <form action="/image?width=100" method="post" enctype="multipart/form-data">
                <input type="file" name="file" accept=".jpeg, .jpg"/>
                <input type="submit" value="Upload"></button>
            </form>
        </body>
    </html>
    "#;

    HttpResponse::Ok().body(html)
}
