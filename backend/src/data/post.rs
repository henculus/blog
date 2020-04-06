use crate::data::Id;
use crate::error::Error;
use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::NaiveDate;
use futures::future::Ready;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct NewPostInfo {
    title: String,
    body: String,
    author: Id,
    is_published: bool,
}
#[derive(Deserialize, Clone)]
pub struct UpdatePostInfo {
    title: Option<String>,
    body: Option<String>,
    is_published: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDate,
    pub edited_at: Option<NaiveDate>,
    pub author: Id,
    pub is_published: bool,
}

impl Responder for Post {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        unimplemented!()
    }
}
