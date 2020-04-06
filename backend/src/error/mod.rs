use crate::database::Error as DatabaseError;
use actix_web::ResponseError;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    DatabaseError(DatabaseError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(e) => e.fmt(f),
        }
    }
}

impl ResponseError for Error {}

// impl Into<ActixError> for Error {
//     fn into(self) -> ActixError {
//         ActixError {cause: Box::new(self)}
//     }
// }

impl From<DatabaseError> for Error {
    fn from(e: DatabaseError) -> Self {
        Error::DatabaseError(e)
    }
}
