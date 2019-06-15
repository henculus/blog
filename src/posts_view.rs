use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonError};

use crate::{Id, ViewResult};
use crate::Database;
use crate::post::{Post, PostData};
use crate::schema::posts::dsl::*;
use crate::schema::users::dsl::*;
use crate::user::{Token, User};

const OFFSET: i64 = 0;
const LIMIT: i64 = 10;

type JsonForm<'a, T> = Result<Json<T>, JsonError<'a>>;

#[post("/posts", format = "json", data = "<post_form>")]
pub fn new_post(post_form: JsonForm<PostData>, conn: Database, token: Token) -> ViewResult<Post> {
    let post = post_form?
        .into_inner()
        .validate()?;

    let query_result = diesel::insert_into(posts)
        .values((post, author.eq(token.username())))
        .get_result(&*conn)?;

    Ok(Json(query_result))
}

#[get("/posts?<limit>&<offset>")]
pub fn get_posts(conn: Database, limit: Option<i64>, offset: Option<i64>) -> ViewResult<Vec<Post>> {
    let query_result = posts
        .filter(published.eq(true))
        .offset(offset.unwrap_or(OFFSET))
        .limit(limit.unwrap_or(LIMIT))
        .load::<Post>(&*conn)?;

    Ok(Json(query_result))
}

#[get("/posts/<post_id>")]
pub fn get_users_post(post_id: Id, conn: Database, token: Token) -> ViewResult<Post> {
    let user: User = users.find(token.username()).get_result(&*conn)?;
    let query_result = Post::belonging_to(&user)
        .filter(id.eq(&post_id))
        .get_result(&*conn)?;
    Ok(Json(query_result))
}

#[get("/posts/<post_id>", rank = 2)]
pub fn get_post(post_id: Id, conn: Database) -> ViewResult<Post> {
    let query_result = posts
        .filter(id.eq(&post_id).and(published.eq(true)))
        .get_result(&*conn)?;
    Ok(Json(query_result))
}

#[put("/posts/<post_id>", format = "application/json", data = "<post_data>")]
pub fn update_post(post_id: Id, post_data: Json<PostData>, conn: Database, token: Token) -> ViewResult<Post> {
    let post_data = post_data.into_inner();
    let original_post = posts.filter(id.eq(post_id).and(author.eq(token.username())));
    let query_result = diesel::update(original_post).set(&post_data).get_result(&*conn)?;
    Ok(Json(query_result))
}

#[put("/posts/<post_id>/publish")]
pub fn publish_post(post_id: Id, conn: Database, token: Token) -> ViewResult<Post> {
    let original_post = posts.filter(id.eq(post_id).and(author.eq(token.username())));
    let query_result = diesel::update(original_post)
        .set(published.eq(true))
        .get_result(&*conn)?;
    Ok(Json(query_result))
}

#[delete("/posts/<post_id>")]
pub fn delete_post(post_id: Id, conn: Database, token: Token) -> ViewResult<usize> {
    let post = posts.filter(id.eq(post_id).and(author.eq(token.username())));
    let query_result = diesel::delete(post).execute(&*conn)?;
    Ok(Json(query_result))
}
