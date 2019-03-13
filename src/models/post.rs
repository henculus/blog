use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{DBConnection, error::*, Id, Model, schema::posts};

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

pub struct PostsTable {
    db_connection: DBConnection,
}

impl Model for PostsTable {
    type Key = i32;
    type Item = Post;
    type NewItem = NewPost;

    fn new(connection: DBConnection) -> Self {
        Self {
            db_connection: connection,
        }
    }

    fn create(&self, post: NewPost) -> Result<Post, ModelError> {
        if post.title.trim() == "" || post.body.trim() == "" {
            return Err(ModelError {
                kind: ModelErrorKind::ValidationError,
                message: "Title or body is empty".to_string(),
            });
        }
        let result = diesel::insert_into(posts::table)
            .values(&post)
            .get_result(&*self.db_connection)?;
        Ok(result)
    }

    fn update(&self, post_id: Id, post: NewPost) -> Result<Id, ModelError> {
        if post.title.trim() == "" || post.body.trim() == "" {
            return Err(ModelError {
                kind: ModelErrorKind::ValidationError,
                message: "Title or body is empty".to_string(),
            });
        }
        let result = diesel::update(posts::table.find(post_id))
            .set(&post)
            .execute(&*self.db_connection)?;
        Ok(result as Id)
    }

    fn get(&self, limit: i64, offset: i64) -> Result<Vec<Post>, ModelError> {
        let result = posts::table
            .offset(offset)
            .limit(limit)
            .load::<Post>(&*self.db_connection)?;
        Ok(result)
    }

    fn get_by_id(&self, post_id: Id) -> Result<Post, ModelError> {
        let result = posts::table
            .find(post_id)
            .first::<Post>(&*self.db_connection)?;
        Ok(result)
    }

    fn delete(&self, post_id: Id) -> Result<i32, ModelError> {
        let result = diesel::delete(posts::table.find(post_id)).execute(&*self.db_connection)?;
        Ok(result as i32)
    }
}
