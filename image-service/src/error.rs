use image::ImageError;

#[derive(Debug)]
pub enum Error {
    ImageError(ImageError),
    ReqwestError(reqwest::Error),
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

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::ReqwestError(reqwest_error)
    }
}