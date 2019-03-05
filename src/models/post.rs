use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::schema::posts;
use crate::models::errors::*;
use crate::models::Id;

#[derive(Queryable, Serialize, Identifiable)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

pub struct PostsTable {
    connection: PgConnection
}

impl PostsTable {
    pub fn instance(connection: PgConnection) -> Self {
        Self { connection }
    }

    pub fn create(&self, post: NewPost) -> Result<Post, CreationError> {
        let result = diesel::insert_into(posts::table)
            .values(&post)
            .get_result(&self.connection)?;
        Ok(result)
    }

    pub fn get(&self, limit: i64, offset: i64) -> Result<Vec<Post>, SelectionError> {
        let result = posts::table
            .offset(offset)
            .limit(limit)
            .load::<Post>(&self.connection)?;
        Ok(result)
    }

    pub fn get_by_id(&self, post_id: Id) -> Result<Post, SelectionError> {
        let result = posts::table
            .find(post_id)
            .first::<Post>(&self.connection)?;
        Ok(result)
    }
}