use crate::error::Error;
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::Ready;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Id(pub i32);

impl Into<Id> for i32 {
    fn into(self) -> Id {
        Id(self)
    }
}

impl Responder for Id {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        unimplemented!()
    }
}
