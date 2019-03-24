use std::fmt;

use rocket::{
    http::{ContentType, Status},
    Request,
    Response, response::Responder,
};
use rocket_contrib::json::{Json, JsonError};
use serde::Serialize;

use crate::models::error::ModelError;

#[derive(Debug, Serialize)]
pub struct ViewError {
    pub status: String,
    pub kind: ViewErrorKind,
    pub resource: Option<String>,
}

impl ViewError {
    pub fn new_unauthorized() -> Self {
        Self {
            status: "error".to_string(),
            kind: ViewErrorKind::Unauthorized,
            resource: None,
        }
    }
}

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ViewErrorKind::ServiceUnavailable => write!(f, "Service {:?} unavailable", self.resource),
            ViewErrorKind::NotFound => write!(f, "Resource {:?} not found", self.resource),
            ViewErrorKind::BadRequest => write!(f, "Bad request: {:?}", self.resource),
            ViewErrorKind::UnprocessableEntity => write!(f, "Unprocessable Entity: {:?}", self.resource),
            ViewErrorKind::Unauthorized => write!(f, "Unauthorized: {:?}", self.resource),
            ViewErrorKind::InternalError => write!(f, "Internal Error: {:?}", self.resource),
            ViewErrorKind::Conflict => write!(f, "Conflict: {:?}", self.resource),
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
            ViewErrorKind::Unauthorized => resp.status(Status::Unauthorized),
            ViewErrorKind::InternalError => resp.status(Status::InternalServerError),
            ViewErrorKind::Conflict => resp.status(Status::Conflict),
        };
        Ok(resp.finalize())
    }
}

impl<'a> From<JsonError<'a>> for ViewError {
    fn from(err: JsonError) -> Self {
        Self {
            status: "error".to_string(),
            kind: ViewErrorKind::UnprocessableEntity,
            resource: match err {
                JsonError::Io(_) => Some("Couldn't read request content".to_string()),
                JsonError::Parse(_, err) => Some(format!("Wrong JSON: '{}'", err.to_string())),
            },
        }
    }
}

impl From<ModelError> for ViewError {
    fn from(err: ModelError) -> Self {
        let mut res = ViewError {
            status: "error".to_string(),
            kind: ViewErrorKind::InternalError,
            resource: None,
        };

        match err {
            ModelError::NotFound(_) => res.kind = ViewErrorKind::NotFound,
            ModelError::Conflict(_) => res.kind = ViewErrorKind::Conflict,
            ModelError::ValidationError(_) => res.kind = ViewErrorKind::UnprocessableEntity,
            ModelError::InvalidCredentials(_) => res.kind = ViewErrorKind::Unauthorized,
            ModelError::DatabaseError(_) => res.kind = ViewErrorKind::ServiceUnavailable,
        }

        res
    }
}

#[derive(Debug, Serialize)]
pub enum ViewErrorKind {
    ServiceUnavailable,
    NotFound,
    BadRequest,
    Conflict,
    UnprocessableEntity,
    Unauthorized,
    InternalError,
}
