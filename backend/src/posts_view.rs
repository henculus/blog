use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonError};

use crate::{Id, ViewResult};
use crate::Database;
use crate::post::{NewPostData, Post, PostDataUpdate};
use crate::schema::posts as posts_schema;
use crate::user::Token;

const OFFSET: i64 = 0;
const LIMIT: i64 = 10;

type JsonForm<'a, T> = Result<Json<T>, JsonError<'a>>;

#[post("/posts", format = "json", data = "<post_form>")]
pub fn new_post(post_form: JsonForm<NewPostData>, conn: Database, token: Token) -> ViewResult<Post> {
    use posts_schema::dsl::*;

    let post = post_form?.into_inner().validate()?;

    let query_result = diesel::insert_into(posts)
        .values((post, author.eq(token.username())))
        .get_result(&*conn)?;

    Ok(Json(query_result))
}

#[get("/posts?<limit>&<offset>", rank = 2)]
pub fn get_posts(conn: Database, limit: Option<i64>, offset: Option<i64>, token: Option<Token>) -> ViewResult<Vec<Post>> {
    let query_result;
    match token {
        Some(t) => {
            query_result = posts_schema::table
                .filter(posts_schema::published.eq(true).or(posts_schema::author.eq(t.username())))
                .offset(offset.unwrap_or(OFFSET))
                .limit(limit.unwrap_or(LIMIT))
                .load::<Post>(&*conn)?;
        }
        None => {
            query_result = posts_schema::table
                .filter(posts_schema::published.eq(true))
                .offset(offset.unwrap_or(OFFSET))
                .limit(limit.unwrap_or(LIMIT))
                .load::<Post>(&*conn)?;
        }
    }
    Ok(Json(query_result))
}

#[get("/posts?<author>&<limit>&<offset>")]
pub fn get_posts_by_author(
    conn: Database,
    author: String,
    limit: Option<i64>,
    offset: Option<i64>,
    token: Option<Token>
) -> ViewResult<Vec<Post>> {
    let query = posts_schema::table.filter(posts_schema::author.eq(&author));
    let result;
    match token {
        Some(t) => {
            if &author == t.username() {
                result = query
                    .offset(offset.unwrap_or(OFFSET))
                    .limit(limit.unwrap_or(LIMIT))
                    .load::<Post>(&*conn)?;
            } else {
                result = query
                    .filter(posts_schema::published.eq(true))
                    .offset(offset.unwrap_or(OFFSET))
                    .limit(limit.unwrap_or(LIMIT))
                    .load::<Post>(&*conn)?;
            }
        }
        None => {
            result = query
                .filter(posts_schema::published.eq(true))
                .offset(offset.unwrap_or(OFFSET))
                .limit(limit.unwrap_or(LIMIT))
                .load::<Post>(&*conn)?;
        }
    }
    Ok(Json(result))
}

#[get("/posts/<post_id>")]
pub fn get_post(post_id: Id, conn: Database, token: Option<Token>) -> ViewResult<Post> {
    use posts_schema::dsl::*;

    let query_result;
    match token {
        Some(t) => {
            query_result = posts
                .filter(id.eq(&post_id))
                .filter(published.eq(true).or(author.eq(t.username())))
                .get_result(&*conn)?;
        }
        None => {
            query_result = posts
                .filter(id.eq(&post_id).and(published.eq(true)))
                .get_result(&*conn)?;
        }
    }
    Ok(Json(query_result))
}

#[patch("/posts/<post_id>", format = "application/json", data = "<post_data>")]
pub fn update_post(
    post_id: Id,
    post_data: Json<PostDataUpdate>,
    conn: Database,
    token: Token,
) -> ViewResult<Post> {
    use posts_schema::dsl::*;

    let post_data = post_data.into_inner();
    let original_post = posts.filter(id.eq(post_id).and(author.eq(token.username())));
    let query_result = diesel::update(original_post)
        .set(&post_data)
        .get_result(&*conn)?;
    Ok(Json(query_result))
}

#[delete("/posts/<post_id>")]
pub fn delete_post(post_id: Id, conn: Database, token: Token) -> ViewResult<usize> {
    use posts_schema::dsl::*;

    let post = posts.filter(id.eq(post_id).and(author.eq(token.username())));
    let query_result = diesel::delete(post).execute(&*conn)?;
    Ok(Json(query_result))
}
