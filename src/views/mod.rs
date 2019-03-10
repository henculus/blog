use rocket::{
    Request,
    request::Form,
    State,
};
use rocket_contrib::json::Json;

use crate::{
    models::{
        Id,
        Model,
        post::*,
        TableManager,
    },
    views::error::*,
};

mod error;

#[post("/posts", format = "application/json", data = "<post>")]
pub fn new_post(post: Json<NewPost>,
                table_manager: State<TableManager>) -> Result<Json<Id>, ViewError> {
    let posts_table: PostsTable = table_manager.get()?;
    posts_table
        .create(post.into_inner())
        .map(|post| Ok(Json(post.id)))?
}

#[derive(FromForm)]
pub struct LimitOffset {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/posts?<cursor..>")]
pub fn get_posts(cursor: Form<LimitOffset>,
                 table_manager: State<TableManager>) -> Result<Json<Vec<Post>>, ViewError> {
    let table: PostsTable = table_manager.get()?;
    table
        .get(cursor.limit.unwrap_or(1), cursor.offset.unwrap_or(0))
        .map(|posts| Ok(Json(posts)))?
}

#[get("/posts/<id>")]
pub fn get_post(id: Id, table_manager: State<TableManager>) -> Result<Json<Post>, ViewError> {
    let posts_table: PostsTable = table_manager.get()?;
    posts_table
        .get_by_id(id)
        .map(|post| Ok(Json(post)))?
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: Id, post: Json<NewPost>,
                   table_manager: State<TableManager>) -> Result<Json<i32>, ViewError> {
    let posts_table: PostsTable = table_manager.get()?;

    posts_table
        .update(id, post.into_inner())
        .map(|row_affected| Ok(Json(row_affected)))?
}

#[delete("/posts/<id>")]
pub fn delete_post(id: Id, table_manager: State<TableManager>) -> Result<Json<i32>, ViewError> {
    let posts_table: PostsTable = table_manager.get()?;

    posts_table
        .delete(id)
        .map(|row_affected| Ok(Json(row_affected)))?
}

#[catch(503)]
pub fn service_unavailable(_: &Request) -> Json<ViewError> {
    let error = ViewError {
        status: "error".to_string(),
        kind: ViewErrorKind::ServiceUnavailable,
        resource: None,
    };
    Json(error)
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ViewError> {
    let error = ViewError {
        status: "error".to_string(),
        kind: ViewErrorKind::NotFound,
        resource: Some(req.to_string()),
    };
    Json(error)
}