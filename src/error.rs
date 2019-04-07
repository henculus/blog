use rocket::{Request, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

type DieselError = diesel::result::Error;
type DatabaseErrorKind = diesel::result::DatabaseErrorKind;

#[derive(Debug)]
pub enum Error {
    UnimplementedError,
    WrongAuthType,
    NoAuthHeader,
    TokenError(jsonwebtoken::errors::Error),
    WrongPassword,
    DieselError(diesel::result::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::TokenError(e) => Some(e),
            Error::DieselError(e) => Some(e),
            _ => None
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::TokenError(e) => write!(f, "{}", e.to_string()),
            Error::DieselError(e) => write!(f, "{}", e.to_string()),
            _ => write!(f, "{:?}", self)
        }
    }
}


impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::DieselError(e)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Error::TokenError(e)
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    pub error: String,
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, request: &Request) -> Result<Response<'a>, Status> {
        let error = ErrorResponse { error: self.to_string() };
        let mut resp = Response::build();
        error!("{:#?}", self);
        match self {
            Error::UnimplementedError => resp.status(Status::NotImplemented),
            Error::WrongAuthType => resp.status(Status::BadRequest),
            Error::NoAuthHeader => resp.status(Status::Unauthorized),
            Error::TokenError(_) => resp.status(Status::Unauthorized),
            Error::WrongPassword => resp.status(Status::Unauthorized),
            Error::DieselError(e) => match e: DieselError {
                DieselError::DatabaseError(k, _) => match k: DatabaseErrorKind {
                    DatabaseErrorKind::UniqueViolation => resp.status(Status::Conflict),
                    DatabaseErrorKind::ForeignKeyViolation => resp.status(Status::Conflict),
                    _ => resp.status(Status::InternalServerError),
                },
                DieselError::NotFound => resp.status(Status::NotFound),
                _ => resp.status(Status::InternalServerError),
            }
        };
        let content = Json(error).respond_to(request)?;
        resp.header(ContentType::JSON).merge(content);
        Ok(resp.finalize())
    }
}