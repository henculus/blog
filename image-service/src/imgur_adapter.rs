use crate::error::Error;
use log::*;
use mime_guess::from_path;
use reqwest::{multipart::Form, multipart::Part, Client};
use serde_json::Value;

const API_URL: &str = "https://api.imgur.com/3/upload";
const CLIENT_ID: &str = "cc27cc3925c6140";

pub fn get_client_id_from_env() -> String {
    std::env::var("IMGUR_CLIENT_ID").unwrap_or_else(|_| {
        warn!("IMGUR_CLIENT_ID variable is not set, using default value");
        CLIENT_ID.into()
    })
}

pub async fn upload(buffer: Vec<u8>, image_name: String) -> Result<String, Error> {
    info!("Uploading image {} to imgur", &image_name);
    let client = Client::new();

    let part = create_part(buffer, image_name).await?;
    let form = Form::new().part("image", part);
    let resp = client
        .post(API_URL)
        .header(
            "Authorization",
            format!("Client-ID {}", get_client_id_from_env()),
        )
        .multipart(form)
        .send()
        .await?
        .error_for_status()?;

    info!("Uploading complete");
    let data = resp.json::<Value>().await?;
    Ok(data["data"]["link"].to_string())
}

async fn create_part(buffer: Vec<u8>, image_name: String) -> Result<Part, Error> {
    info!("Creating part for image");
    let mime: String = from_path(&image_name)
        .first()
        .map_or("image/jpeg".to_string(), |m| m.to_string());

    Ok(Part::bytes(buffer)
        .file_name(image_name)
        .mime_str(mime.as_str())?)
}
