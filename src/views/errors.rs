use serde::Serialize;
use std::fmt;
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket::http::Status;
use crate::models::errors::ModelError;

#[derive(Debug, Serialize)]
pub enum ViewError {
    ServiceUnavailable,
    NotFound,
}

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewError::ServiceUnavailable => write!(f, "Service unavailable"),
            ViewError::NotFound => write!(f, "Not found")
        }
    }
}

impl<'a> Responder<'a> for ViewError {
    fn respond_to(self, request: &Request) -> Result<Response<'a>, Status> {
        match self {
            ViewError::NotFound => Err(Status::NotFound),
            ViewError::ServiceUnavailable => Err(Status { code: 503, reason: "Service unavailable" })
        }
    }
}

impl From<ModelError> for ViewError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::DBConnectionError => ViewError::ServiceUnavailable,
            ModelError::OperationError(_) => ViewError::NotFound
        }
    }
}