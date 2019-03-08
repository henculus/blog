use serde::Serialize;
use std::fmt;
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket::http::{Status, ContentType};
use crate::models::error::ModelError;
use rocket_contrib::json::Json;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct ViewError {
    pub status: String,
    pub kind: ViewErrorKind,
    pub resource: Option<String>,
}

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ViewErrorKind::ServiceUnavailable => write!(f, "Service {:?} unavailable", self.resource),
            ViewErrorKind::NotFound => write!(f, "Resource {:?} not found", self.resource)
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
            ViewErrorKind::ServiceUnavailable => resp.status(Status { code: 503, reason: "Service Unavailable" })
        };
        Ok(resp.finalize())
    }
}

impl From<ModelError> for ViewError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::DBConnectionError => ViewError {
                status: "error".to_string(),
                kind: ViewErrorKind::ServiceUnavailable,
                resource: Some("database".to_string())
            },
            ModelError::OperationError(err) => ViewError {
                status: "error".to_string(),
                kind: ViewErrorKind::NotFound,
                resource: Some(err.description().to_string())
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ViewErrorKind {
    ServiceUnavailable,
    NotFound,
}
