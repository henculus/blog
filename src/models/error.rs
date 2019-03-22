use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ModelError {
    pub kind: ModelErrorKind,
    pub message: String,
}

#[derive(Debug)]
pub enum ModelErrorKind {
    OperationError,
    DBConnectionError,
    ValidationError,
    InvalidCredentials,
}

impl Error for ModelError {}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ModelErrorKind::OperationError => write!(f, "Cannot execute operation: {:?}", self.message),
            ModelErrorKind::DBConnectionError => write!(f, "Cannot connect to the database"),
            ModelErrorKind::ValidationError => write!(f, "Cannot validate fields"),
            ModelErrorKind::InvalidCredentials => write!(f, "Wrong user's credentials")
        }
    }
}

impl From<r2d2::Error> for ModelError {
    fn from(err: r2d2::Error) -> Self {
        Self {
            kind: ModelErrorKind::DBConnectionError,
            message: err.to_string(),
        }
    }
}

impl From<diesel::result::Error> for ModelError {
    fn from(err: diesel::result::Error) -> Self {
        Self {
            kind: ModelErrorKind::OperationError,
            message: err.to_string(),
        }
    }
}
