use crate::api::JsonResponse;
use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::database::{change_post, retrieve_post, retrieve_posts, DatabaseClient, create_post, remove_post};
use crate::error::Error;
use actix_web::{
    delete, get, patch, post, put,
    web::{Data, Json, Path},
};

#[get("/posts/{post_id}/")]
pub async fn get_post(client: Data<DatabaseClient>, id: Path<Id>) -> JsonResponse<Post, Error> {
    let raw_id = id.into_inner();
    let post = client.execute(retrieve_post(raw_id)).await?;
    Ok(Json(post))
}

#[get("/posts/")]
pub async fn get_posts(client: Data<DatabaseClient>) -> JsonResponse<Vec<Post>, Error> {
    let page = 1;
    let page_size = 10;
    let posts = client.execute(retrieve_posts(page, page_size)).await?;
    Ok(Json(posts))
}

#[post("/posts/")]
pub async fn add_post(client: Data<DatabaseClient>, post_info: Json<NewPostInfo>) -> JsonResponse<Id, Error> {
    let id = client.execute(create_post(post_info.into_inner())).await?;
    Ok(Json(id))
}

#[delete("/posts/{post_id}/")]
pub async fn delete_post(client: Data<DatabaseClient>, id: Path<Id>) -> JsonResponse<(), Error> {
    client.execute(remove_post(id.into_inner())).await?;
    Ok(Json(()))
}

#[put("/posts/{post_id}/")]
pub async fn replace_post(client: Data<DatabaseClient>, id: Path<Id>, post_info: Json<NewPostInfo>) -> JsonResponse<(), Error> {
    unimplemented!()
}

#[patch("/posts/{post_id}/")]
pub async fn update_post(client: Data<DatabaseClient>, id: Path<Id>, post_info: Json<UpdatePostInfo>) -> JsonResponse<(), Error> {
    client.execute(change_post(id.into_inner(), post_info.into_inner())).await?;
    Ok(Json(()))
}
