use crate::{error::Error, img::resize, imgur_adapter::upload, multipart_reader::read_buffer};
use actix_multipart::Multipart;
use actix_web::{web::Query, HttpResponse};
use log::*;
use serde::{Deserialize, Serialize};
use futures::future::try_join;

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

pub async fn process_image(
    info: Query<ProcessImageInfo>,
    multipart: Multipart,
) -> Result<HttpResponse, Error> {
    info!("Request to resize image");
    let width = info.width;
    let height = info.height;

    let buffer = read_buffer(multipart).await?;
    let resized = resize(buffer.clone(), width, height).await?;
    let high = upload(buffer, "high_res.jpg".into());
    let low = upload(resized, "low_res.jpg".into());
    let (high_link, low_ling) = try_join(high, low).await?;

    Ok(HttpResponse::Ok().json(Links {
        high_resolution: high_link,
        low_resolution: low_ling
    }))
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
