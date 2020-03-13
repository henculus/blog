use crate::error::Error;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::Ready;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Id(i32);

impl Into<Id> for i32 {
    fn into(self) -> Id {
        Id(self)
    }
}