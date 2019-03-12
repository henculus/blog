use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ModelError {
    OperationError(diesel::result::Error),
    DBConnectionError,
}

impl Error for ModelError {}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelError::OperationError(_) => write!(f, "Cannot execute operation"),
            ModelError::DBConnectionError => write!(f, "Cannot connect to the database"),
        }
    }
}

impl From<r2d2::Error> for ModelError {
    fn from(_: r2d2::Error) -> Self {
        ModelError::DBConnectionError
    }
}

impl From<diesel::result::Error> for ModelError {
    fn from(err: diesel::result::Error) -> Self {
        ModelError::OperationError(err)
    }
}
