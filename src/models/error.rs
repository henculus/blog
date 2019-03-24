use std::error;
use std::fmt;

use diesel::result::DatabaseErrorKind;

#[derive(Debug)]
pub enum ModelError {
    NotFound(Option<String>),
    Conflict(Option<String>),
    ValidationError(Option<String>),
    InvalidCredentials(Option<String>),
    DatabaseError(Option<String>),
}

impl error::Error for ModelError {}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelError::NotFound(m) => write!(f, "Row not found: {:?}", m),
            ModelError::Conflict(m) => write!(f, "Operations conflict: {:?}", m),
            ModelError::ValidationError(m) => write!(f, "Validation Error: {:?}", m),
            ModelError::InvalidCredentials(m) => write!(f, "Access denied: {:?}", m),
            ModelError::DatabaseError(m) => write!(f, "Database Error: {:?}", m),
        }
    }
}

impl From<diesel::result::Error> for ModelError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::InvalidCString(_) => {
                ModelError::DatabaseError(Some("Internal Error: invalid CString".to_string()))
            }
            diesel::result::Error::DatabaseError(kind, _) => {
                match kind {
                    DatabaseErrorKind::UniqueViolation => {
                        ModelError::Conflict(Some("Unique violation".to_string()))
                    }
                    DatabaseErrorKind::ForeignKeyViolation => {
                        ModelError::Conflict(Some("Foreign key violation".to_string()))
                    }
                    DatabaseErrorKind::UnableToSendCommand => {
                        ModelError::DatabaseError(Some("Unable to send command".to_string()))
                    }
                    DatabaseErrorKind::SerializationFailure => {
                        ModelError::DatabaseError(Some("Serialization failure".to_string()))
                    }
                    _ => {
                        ModelError::DatabaseError(Some("Unknown".to_string()))
                    }
                }
            }
            diesel::result::Error::NotFound =>
                ModelError::NotFound(Some("Row not found".to_string())),

            diesel::result::Error::QueryBuilderError(_) =>
                ModelError::DatabaseError(Some("Invalid diesel query".to_string())),

            diesel::result::Error::DeserializationError(_) =>
                ModelError::DatabaseError(Some("Deserialization error: Incorrect struct type".to_string())),

            diesel::result::Error::SerializationError(_) =>
                ModelError::DatabaseError(Some("Serialization error: Incorrect type".to_string())),

            _ =>
                ModelError::DatabaseError(Some("Unknown".to_string())),
        }
    }
}

impl From<r2d2::Error> for ModelError {
    fn from(_: r2d2::Error) -> Self {
        ModelError::DatabaseError(Some("Database connection pool error (r2d2)".to_string()))
    }
}
