use image::ImageError;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum Error {
    ImageError(ImageError),
    ReqwestError(ReqwestError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ImageError(e) => Some(e),
            Error::ReqwestError(e) => Some(e)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ImageError(e) => e.fmt(f),
            Error::ReqwestError(e) => e.fmt(f)
        }
    }
}

impl From<ImageError> for Error {
    fn from(image_error: ImageError) -> Self {
        Error::ImageError(image_error)
    }
}

impl From<ReqwestError> for Error {
    fn from(reqwest_error: ReqwestError) -> Self {
        Error::ReqwestError(reqwest_error)
    }
}