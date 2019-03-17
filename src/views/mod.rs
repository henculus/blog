use std::io;
use std::path::{Path, PathBuf};

use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;

use crate::views::error::*;

mod error;
mod tests;
pub mod posts;
pub mod users;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/dist/index.html")
}

#[get("/<file..>", rank = 1)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/dist/").join(file)).ok()
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