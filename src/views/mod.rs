use std::io;
use std::path::{Path, PathBuf};

use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;

use crate::views::error::*;

mod error;
pub mod posts;
mod tests;
pub mod users;

#[get("/", rank = 10)]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/dist/index.html")
}

// TODO: Handle forwarding to this request from /api routes, may be with request guard
#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    match NamedFile::open(Path::new("static/dist/").join(file)) {
        Ok(file) => Some(file),
        Err(_) => NamedFile::open("static/dist/index.html").ok(),
    }
}

#[catch(503)]
pub fn service_unavailable(_: &Request) -> Json<ViewError> {
    let error = ViewError {
        status: "error".to_string(),
        kind: ViewErrorKind::ServiceUnavailable,
        resource: None,
    };
    Json(error)
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ViewError> {
    let error = ViewError {
        status: "error".to_string(),
        kind: ViewErrorKind::NotFound,
        resource: Some(req.to_string()),
    };
    Json(error)
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<ViewError> {
    let error = ViewError {
        status: "error".to_string(),
        kind: ViewErrorKind::UnprocessableEntity,
        resource: Some(req.to_string()),
    };
    Json(error)
}

#[catch(400)]
pub fn bad_request(req: &Request) -> Json<ViewError> {
    let error = ViewError {
        status: "error".to_string(),
        kind: ViewErrorKind::BadRequest,
        resource: Some(req.to_string()),
    };
    Json(error)
}
