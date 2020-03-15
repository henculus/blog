use crate::error::Error;
use actix_web::body::Body;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Responder};
use futures::future::Ready;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Id(i32);

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
