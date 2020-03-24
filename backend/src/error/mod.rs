use crate::database::Error as DatabaseError;
use actix_web::Error as ActixError;
pub enum Error {}

impl Into<ActixError> for Error {
    fn into(self) -> ActixError {
        unimplemented!()
    }
}

impl From<DatabaseError> for Error {
    fn from(_: DatabaseError) -> Self {
        unimplemented!()
    }
}
