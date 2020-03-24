use crate::api::JsonResponse;
use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::database::{retrieve_post, DatabaseClient};
use crate::error::Error;
use actix_web::{
    delete, get, patch, post, put,
    web::{Data, Json, Path},
};

#[get("/posts/{post_id}")]
pub async fn get_post(client: Data<DatabaseClient>, id: Path<Id>) -> JsonResponse<Post, Error> {
    let raw_id = id.into_inner();
    let post = client.execute(retrieve_post(raw_id)).await?;
    Ok(Json(post))
}

#[get("/posts/")]
pub async fn get_posts() -> JsonResponse<Vec<Post>, Error> {
    unimplemented!()
}

#[post("/posts/")]
pub async fn add_post(post_info: Json<NewPostInfo>) -> JsonResponse<Id, Error> {
    unimplemented!()
}

#[delete("/posts/{post_id}")]
pub async fn delete_post(id: Path<Id>) -> JsonResponse<(), Error> {
    unimplemented!()
}

#[put("/posts/{post_id}")]
pub async fn replace_post(id: Path<Id>, post_info: Json<NewPostInfo>) -> JsonResponse<(), Error> {
    unimplemented!()
}

#[patch("/posts/{post_id}")]
pub async fn update_post(id: Path<Id>, post_info: Json<UpdatePostInfo>) -> JsonResponse<(), Error> {
    unimplemented!()
}
