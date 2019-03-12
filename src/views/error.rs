use std::{error::Error, fmt};

use rocket::{
    http::{ContentType, Status},
    Request,
    Response, response::Responder,
};
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::models::error::{ModelError, ModelErrorKind};

#[derive(Debug, Serialize)]
pub struct ViewError {
    pub status: String,
    pub kind: ViewErrorKind,
    pub resource: Option<String>,
}

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ViewErrorKind::ServiceUnavailable => {
                write!(f, "Service {:?} unavailable", self.resource)
            }
            ViewErrorKind::NotFound => write!(f, "Resource {:?} not found", self.resource),
            ViewErrorKind::BadRequest => write!(f, "Bad request: {:?}", self.resource),
            ViewErrorKind::UnprocessableEntity => write!(f, "Unprocessable Entity: {:?}", self.resource)
        }
    }
}

impl<'a> Responder<'a> for ViewError {
    fn respond_to(self, request: &Request) -> Result<Response<'a>, Status> {
        let content = Json(&self).respond_to(request)?;
        let mut resp = Response::build();
        resp.header(ContentType::JSON).merge(content);
        match self.kind {
            ViewErrorKind::NotFound => resp.status(Status::NotFound),
            ViewErrorKind::ServiceUnavailable => resp.status(Status::ServiceUnavailable),
            ViewErrorKind::BadRequest => resp.status(Status::BadRequest),
            ViewErrorKind::UnprocessableEntity => resp.status(Status::UnprocessableEntity),
        };
        Ok(resp.finalize())
    }
}

impl From<ModelError> for ViewError {
    fn from(err: ModelError) -> Self {
        match err.kind {
            ModelErrorKind::DBConnectionError => ViewError {
                status: "error".to_string(),
                kind: ViewErrorKind::ServiceUnavailable,
                resource: Some("database".to_string()),
            },
            ModelErrorKind::OperationError => ViewError {
                status: "error".to_string(),
                kind: ViewErrorKind::NotFound,
                resource: Some(err.description().to_string()),
            },
            ModelErrorKind::ValidationError => ViewError {
                status: "error".to_string(),
                kind: ViewErrorKind::UnprocessableEntity,
                resource: Some(err.message),
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ViewErrorKind {
    ServiceUnavailable,
    NotFound,
    BadRequest,
    UnprocessableEntity,
}
