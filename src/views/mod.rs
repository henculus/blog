use rocket::Request;
use rocket_contrib::json::Json;

use crate::views::error::*;

mod error;
mod tests;
pub mod posts;

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