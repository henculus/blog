use actix_web::ResponseError;
use deadpool_postgres::PoolError as DeadpoolError;
use serde::export::Formatter;
use tokio_postgres::Error as PostgresError;
#[derive(Debug)]
pub enum Error {
    PoolError(DeadpoolError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PoolError(e) => e.fmt(f),
        }
    }
}

impl ResponseError for Error {}

impl From<DeadpoolError> for Error {
    fn from(e: DeadpoolError) -> Self {
        Error::PoolError(e)
    }
}

impl From<PostgresError> for Error {
    fn from(e: PostgresError) -> Self {
        unimplemented!()
    }
}
