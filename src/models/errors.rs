use std::error;
use std::fmt;
use std::error::Error;


// TODO: Implement all errors in
pub type UpdateError = CreationError;
pub type DeleteError = CreationError;

#[derive(Debug)]
pub struct CreationError(diesel::result::Error);

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot create entity")
    }
}

impl Error for CreationError {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<diesel::result::Error> for CreationError {
    fn from(err: diesel::result::Error) -> Self {
        Self(err)
    }
}


#[derive(Debug)]
pub struct SelectionError(diesel::result::Error);

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot select entry: {}", self.0.description())
    }
}

impl Error for SelectionError {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<diesel::result::Error> for SelectionError {
    fn from(err: diesel::result::Error) -> Self {
        Self(err)
    }
}

#[derive(Debug)]
pub struct DBConnectionError;

impl fmt::Display for DBConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot connect to the database")
    }
}

impl error::Error for DBConnectionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<r2d2::Error> for DBConnectionError {
    fn from(err: r2d2::Error) -> Self {
        Self
    }
}

impl From<diesel::ConnectionError> for DBConnectionError {
    fn from(err: diesel::ConnectionError) -> Self {
        Self
    }
}
