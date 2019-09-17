use crate::error::Error;
use futures::future::result;
use futures::Future;
use log::*;
use image::{guess_format, DynamicImage, FilterType, ImageFormat};

pub fn resize(
    buffer: &[u8],
    width: u32,
    height: u32,
) -> impl Future<Item=Vec<u8>, Error=Error> {
    buffer_to_image(buffer)
        .map(move |img| {
            info!("Resizing image");
            img.resize(
                width,
                height,
                FilterType::Nearest,
            )
        })
        .join(result(guess_format(buffer)).from_err())
        .and_then(|(img, fmt)| {
            image_to_buffer(img, fmt)
        })
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

fn buffer_to_image(buffer: &[u8]) -> impl Future<Item=DynamicImage, Error=Error> {
    info!("Creating DynamicImage from buffer");
    result(image::load_from_memory(buffer)).from_err()
}

