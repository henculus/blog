use crate::data::Id;
use crate::data::User;
use crate::error::Error;
use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::Ready;

pub struct NewPostInfo {
    title: String,
    body: String,
    author: Id,
    is_published: bool,
}

pub struct UpdatePostInfo {
    title: Option<String>,
    body: Option<String>,
    is_published: Option<String>
}

pub struct Post {
    id: Id,
    title: String,
    body: String,
    created_at: String,
    edited_at: String,
    author: User,
    is_published: bool,
}

impl Responder for Post {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        unimplemented!()
    }
}
