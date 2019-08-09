use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::Id;
use crate::schema::posts;
use crate::user::User;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, AsChangeset, Deserialize, Insertable)]
#[belongs_to(User, foreign_key = "author")]
#[table_name = "posts"]
pub struct Post {
    id: Id,
    title: String,
    body: String,
    author: String,
    published: bool,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPostData {
    title: String,
    body: String,
}

#[derive(AsChangeset, Insertable, Deserialize, Serialize)]
#[table_name = "posts"]
pub struct PostDataUpdate {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    published: Option<bool>,
}

impl NewPostData {
    pub fn validate(self) -> Result<Self, Error> {
        if self.title.trim() == "" {
            return Err(Error::EmptyTitle);
        }
        if self.body.trim() == "" {
            return Err(Error::EmptyBody);
        }
        Ok(self)
    }
}
