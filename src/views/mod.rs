mod errors;

use rocket::request::Form;
use rocket_contrib::json::Json;
use crate::models::post::*;
use crate::models::{Id, DBPool};
use rocket::{Request, State};
use crate::views::errors::NotFoundError;

#[post("/posts", format = "application/json", data = "<post>")]
pub fn new_post(post: Json<NewPost>, db: State<DBPool>) -> Option<Json<Post>> {
    let connection = db.get().unwrap();
    let posts_table = PostsTable::instance(&*connection);
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
pub fn get_posts(cursor: Form<LimitOffset>, db: State<DBPool>) -> Option<Json<Vec<Post>>> {
    let connection = db.get().unwrap();
    let posts_table = PostsTable::instance(&*connection);
    posts_table
        .get(cursor.limit.unwrap_or(1), cursor.offset.unwrap_or(0))
        .map(|posts| Json(posts))
        .ok()
}

#[get("/posts/<id>")]
pub fn get_post(id: Id, db: State<DBPool>) -> Option<Json<Post>> {
    let connection = db.get().unwrap();
    let posts_table = PostsTable::instance(&*connection);
    posts_table
        .get_by_id(id)
        .map(|post| Json(post))
        .ok()
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: Id, post: Json<NewPost>, db: State<DBPool>) -> Option<Json<Id>> {
    let connection = db.get().unwrap();
    let posts_table = PostsTable::instance(&*connection);
    posts_table
        .update(id, post.into_inner())
        .map(|id| Json(id))
        .ok()
}

#[delete("/posts/<id>")]
pub fn delete_post(id: Id, db: State<DBPool>) -> Option<Json<Id>> {
    let connection = db.get().unwrap();
    let posts_table = PostsTable::instance(&*connection);
    posts_table
        .delete(id)
        .map(|id| Json(id))
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