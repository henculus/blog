use actix_http::error::Error as ActixError;
use actix_multipart::MultipartError;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use image::ImageError;
use log::*;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum Error {
    ImageError(ImageError),
    ReqwestError(ReqwestError),
    MultipartError(MultipartError),
    ActixError(ActixError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ImageError(e) => Some(e),
            Error::ReqwestError(e) => Some(e),
            Error::MultipartError(_e) => None,
            Error::ActixError(e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ImageError(e) => e.fmt(f),
            Error::ReqwestError(e) => e.fmt(f),
            Error::MultipartError(e) => e.fmt(f),
            Error::ActixError(e) => e.fmt(f),
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

impl From<MultipartError> for Error {
    fn from(multipart_error: MultipartError) -> Self {
        Error::MultipartError(multipart_error)
    }
}

impl From<ActixError> for Error {
    fn from(actix_error: ActixError) -> Self {
        Error::ActixError(actix_error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::ImageError(e) => error!("Image error: {}", e),
            Error::ReqwestError(e) => error!("Reqwest error: {}", e),
            Error::MultipartError(e) => error!("Multipart error: {}", e),
            Error::ActixError(e) => error!("Actix Http Error: {}", e),
        };
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
