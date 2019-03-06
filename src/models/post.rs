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

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

pub struct PostsTable<'a> {
    connection: &'a PgConnection,
}

impl<'a> PostsTable<'a> {
    pub fn instance(connection: &'a PgConnection) -> Self {
        Self { connection }
    }

    pub fn create(&self, post: NewPost) -> Result<Post, ModelError> {
        let result = diesel::insert_into(posts::table)
            .values(&post)
            .get_result(self.connection)?;
        Ok(result)
    }

    pub fn delete(&self, post_id: Id) -> Result<Id, ModelError> {
        let result = diesel::delete(posts::table.find(post_id))
            .execute(self.connection)?;
        Ok(result as Id)
    }

    pub fn update(&self, post_id: Id, post: NewPost) -> Result<Id, ModelError> {
        let result = diesel::update(posts::table.find(post_id))
            .set(&post)
            .execute(self.connection)?;
        Ok(result as Id)
    }

    pub fn get(&self, limit: i64, offset: i64) -> Result<Vec<Post>, ModelError> {
        let result = posts::table
            .offset(offset)
            .limit(limit)
            .load::<Post>(self.connection)?;
        Ok(result)
    }

    pub fn get_by_id(&self, post_id: Id) -> Result<Post, ModelError> {
        let result = posts::table
            .find(post_id)
            .first::<Post>(self.connection)?;
        Ok(result)
    }
}