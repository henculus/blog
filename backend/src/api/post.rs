use crate::data::{Id, Post, NewPostInfo, UpdatePostInfo};
use crate::error::Error;
use actix_web::{get, web};

#[get("/posts/{post_id}")]
pub async fn get_post(id: web::Path<Id>) -> Result<Post, Error> {
    unimplemented!()
}

#[get("/posts/")]
pub async fn get_posts() -> Result<Vec<Post>, Error> {
    unimplemented!()
}

#[post("/posts/")]
pub async fn add_post(post_info: web::Json<NewPostInfo>) -> Result<Id, Error> {
    unimplemented!()
}

#[delete("/posts/{post_id}")]
pub async fn delete_post(id: web::Path<Id>) -> Result<(), Error> {
    unimplemented!()
}

#[put("/posts/{post_id}")]
pub async fn replace_post(id: web::Path<Id>, post_info: web::Json<NewPostInfo>) -> Result<(), Error> {
    unimplemented!()
}

#[patch("/posts/{post_id}")]
pub async fn update_post(id: web::Path<Id>, post_info: web::Json<UpdatePostInfo>) -> Result<(), Error> {
    unimplemented!()
}