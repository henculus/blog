use crate::error::Error;
use image::{guess_format, imageops::FilterType, DynamicImage, ImageFormat};
use log::*;

pub async fn resize(buffer: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, Error> {
    let img = buffer_to_image(buffer.clone()).await?;
    info!("Resizing image");
    let resized = img.resize(width, height, FilterType::Nearest);
    info!("Resizing complete");
    let format = guess_format(buffer.as_slice())?;
    image_to_buffer(resized, format).await
}

async fn image_to_buffer(img: DynamicImage, format: ImageFormat) -> Result<Vec<u8>, Error> {
    info!("Loading image to buffer");
    let mut buf = Vec::new();
    img.write_to(&mut buf, format)?;
    Ok(buf)
}

async fn buffer_to_image(buffer: Vec<u8>) -> Result<DynamicImage, Error> {
    info!("Creating DynamicImage from buffer");
    Ok(image::load_from_memory(buffer.as_slice())?)
}
