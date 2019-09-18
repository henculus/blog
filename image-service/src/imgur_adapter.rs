use crate::error::Error;
use futures::future::result;
use futures::Future;
use log::*;
use mime_guess::from_path;
use reqwest::r#async::{multipart::Form, multipart::Part, *};

const API_URL: &str = "https://api.imgur.com/3/upload";
const CLIENT_ID: &str = "cc27cc3925c6140";

pub fn get_client_id_from_env() -> String {
    std::env::var("IMGUR_CLIENT_ID").unwrap_or_else(|_| {
        warn!("IMGUR_CLIENT_ID variable is not set, using default value");
        CLIENT_ID.into()
    })
}

pub fn upload(buffer: Vec<u8>, image_name: String) -> impl Future<Item=String, Error=Error> {
    info!("Uploading image {} to imgur", &image_name);
    let client = Client::new();

    create_part(buffer, image_name)
        .and_then(move |part| {
            let form = Form::new().part("image", part);
            client
                .post(API_URL)
                .header("Authorization", format!("Client-ID {}", get_client_id_from_env()))
                .multipart(form)
                .send()
                .from_err()
        })
        .and_then(|mut r| {
            info!("Uploading complete");
            r.json().from_err()
        })
        .map(|data: serde_json::Value| data["data"]["link"].to_string())
}

fn create_part(buffer: Vec<u8>, image_name: String) -> impl Future<Item=Part, Error=Error> {
    info!("Creating part for image");
    let mime: String = from_path(&image_name)
        .first()
        .map_or("image/jpeg".to_string(), |m| m.to_string());

    result(
        Part::bytes(buffer)
            .file_name(image_name)
            .mime_str(mime.as_str()),
    )
        .from_err()
}
