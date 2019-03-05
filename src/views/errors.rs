use serde::Serialize;
use std::fmt;
use std::error::*;

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    pub code: i16,
    pub status: String,
    pub message: String,
    pub resource_description: Option<String>,
}

impl Default for NotFoundError {
    fn default() -> Self {
        Self {
            code: 404,
            status: String::from("error"),
            message: String::from("resource not found"),
            resource_description: None,
        }
    }
}

impl Error for NotFoundError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Not Found Error \n Status: {} \n Message: {} \n Resource description: {:?}",
               self.status,
               self.message,
               self.resource_description)
    }
}

