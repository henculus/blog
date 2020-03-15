use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::error::Error;
use actix_web::{delete, get, patch, post, put, web::Json, web::Path};

#[get("/posts/{post_id}")]
pub async fn get_post(id: Path<Id>) -> Result<Post, Error> {
    unimplemented!()
}

#[get("/posts/")]
pub async fn get_posts() -> Result<Json<Vec<Post>>, Error> {
    unimplemented!()
}

#[post("/posts/")]
pub async fn add_post(post_info: Json<NewPostInfo>) -> Result<Id, Error> {
    unimplemented!()
}

#[delete("/posts/{post_id}")]
pub async fn delete_post(id: Path<Id>) -> Result<Json<()>, Error> {
    unimplemented!()
}

#[put("/posts/{post_id}")]
pub async fn replace_post(id: Path<Id>, post_info: Json<NewPostInfo>) -> Result<Json<()>, Error> {
    unimplemented!()
}

#[patch("/posts/{post_id}")]
pub async fn update_post(id: Path<Id>, post_info: Json<UpdatePostInfo>) -> Result<Json<()>, Error> {
    unimplemented!()
}
