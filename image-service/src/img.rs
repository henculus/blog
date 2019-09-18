use crate::error::Error;
use futures::future::result;
use futures::Future;
use image::{guess_format, DynamicImage, FilterType, ImageFormat};
use log::*;

pub fn resize(
    buffer: Vec<u8>,
    width: u32,
    height: u32,
) -> impl Future<Item=Vec<u8>, Error=Error> {
    buffer_to_image(buffer.clone())
        .map(move |img| {
            info!("Resizing image");
            let resized = img.resize(width, height, FilterType::Nearest);
            info!("Resizing complete");
            resized
        })
        .join(result(guess_format(buffer.as_slice())).from_err())
        .and_then(|(img, fmt)| image_to_buffer(img, fmt))
}

fn image_to_buffer(
    img: DynamicImage,
    format: ImageFormat,
) -> impl Future<Item=Vec<u8>, Error=Error> {
    info!("Loading image to buffer");
    let mut buf = Vec::new();
    result(img.write_to(&mut buf, format))
        .map(|_| buf)
        .from_err()
}

fn buffer_to_image(buffer: Vec<u8>) -> impl Future<Item=DynamicImage, Error=Error> {
    info!("Creating DynamicImage from buffer");
    result(image::load_from_memory(buffer.as_slice())).from_err()
}
