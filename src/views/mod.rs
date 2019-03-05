mod errors;

use rocket::request::Form;
use rocket_contrib::json::Json;
use crate::models::post::*;
use crate::models::connection;
use crate::models::Id;
use rocket::Request;
use crate::views::errors::NotFoundError;

#[post("/posts", format = "application/json", data = "<post>")]
pub fn new_post(post: Json<NewPost>) -> Option<Json<Post>> {
    let connection = connection();
    let posts_table = PostsTable::instance(connection);
    posts_table
        .create(post.into_inner())
        .map(|post| Json(post))
        .ok()
}

#[derive(FromForm)]
pub struct LimitOffset {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/posts?<cursor..>")]
pub fn get_posts(offlim: Form<LimitOffset>) -> Option<Json<Vec<Post>>> {
    let connection = connection();
    let posts_table = PostsTable::instance(connection);
    posts_table
        .get(offlim.limit.unwrap_or(1), offlim.offset.unwrap_or(0))
        .map(|posts| Json(posts))
        .ok()
}

#[get("/posts/<id>")]
pub fn get_post(id: Id) -> Option<Json<Post>> {
    let connection = connection();
    let posts_table = PostsTable::instance(connection);
    posts_table
        .get_by_id(id)
        .map(|post| Json(post))
        .ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<NotFoundError> {
    Json(
        NotFoundError {
            resource_description: Some(req.to_owned().to_string()),
            ..Default::default()
        }
    )
}