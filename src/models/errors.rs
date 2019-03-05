use std::error;
use std::fmt;
use std::error::Error;

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
