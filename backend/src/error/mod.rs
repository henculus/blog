use actix_web::Error as ActixError;

pub enum Error {}

impl Into<ActixError> for Error {
    fn into(self) -> ActixError {
        unimplemented!()
    }
}
