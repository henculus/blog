use rocket::{request::Form, State};
use rocket_contrib::json::{Json, JsonError};

use crate::{
    DBConn,
    models::{Id, Model, post::*, user::*},
    views::error::*,
};

#[post("/posts", format = "application/json", data = "<post>")]
pub fn new_post(
    post: Result<Json<NewPost>, JsonError>,
    conn: DBConn,
) -> Result<Json<Id>, ViewError> {
    let post = post?;
    let table = PostsTable(&conn);
    table
        .create(post.into_inner())
        .map(|post| Ok(Json(post.id)))?
}

#[derive(FromForm)]
pub struct LimitOffset {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/posts?<cursor..>")]
pub fn get_posts(cursor: Form<LimitOffset>, conn: DBConn) -> Result<Json<Vec<Post>>, ViewError> {
    let table = PostsTable(&conn);
    table
        .get(cursor.limit.unwrap_or(10), cursor.offset.unwrap_or(0))
        .map(|posts| Ok(Json(posts)))?
}

#[get("/posts/<id>")]
pub fn get_post(id: Id, conn: DBConn) -> Result<Json<Post>, ViewError> {
    let posts_table = PostsTable(&conn);
    posts_table.get_by_id(id).map(|post| Ok(Json(post)))?
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update_post(
    id: Id,
    post: Result<Json<NewPost>, JsonError>,
    conn: DBConn,
) -> Result<Json<i32>, ViewError> {
    let post = post?;
    let posts_table = PostsTable(&conn);

    posts_table
        .update(id, post.into_inner())
        .map(|row_affected| Ok(Json(row_affected)))?
}

#[delete("/posts/<id>")]
pub fn delete_post(id: Id, conn: DBConn) -> Result<Json<i32>, ViewError> {
    let posts_table = PostsTable(&conn);

    posts_table
        .delete(id)
        .map(|row_affected| Ok(Json(row_affected)))?
}
