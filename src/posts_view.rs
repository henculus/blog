use diesel::prelude::*;
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonError};

use crate::Database;
use crate::models::schema::posts::dsl::*;
use crate::views::{Id, Result};

const OFFSET: i32 = 10;
const LIMIT: i32 = 0;


#[post("/posts", format = "json", data = "<post>")]
pub fn new_post(post: Json<NewPost>, conn: Database, user: User) -> Result<Post> {
    diesel::insert_into(posts)
        .values(post.into_inner())
        .execute(&*conn)?
}

#[get("/posts?<limit>&<offset>")]
pub fn get_posts(db: Database, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Post>> {
    posts
        .filter(published.eq(true))
        .offset(offset.unwrap_or(OFFSET))
        .limit(limit.unwrap_or(LIMIT))
        .order(views.desc())
        .load::<Post>(&db)?
}

#[get("/posts/<id>")]
pub fn get_post(id: Id, conn: Database, user: Option<User>) -> Result<Post> {
    let query = match user {
        Some(user) => posts.filter(author.eq(user.id)),
        None => posts.filter(published.eq(true))
    };

    query
        .filter(id.eq(&id))
        .load::<Post>(&conn)?
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: Id, post: Json<NewPost>, conn: Database, user: User) -> Result<Post> {
    diesel::update(
        posts.filter(id.eq(post_id).and(author.eq(user.id)))
    )
        .set(&post)
        .execute(&db)?
}

#[delete("/posts/<id>")]
pub fn delete_post(id: Id, conn: Database, user: User) -> Result<()> {
    diesel::delete(posts.find(id)).execute(&db)?
}
