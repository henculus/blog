use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::Id;
use crate::schema::posts;
use crate::user::User;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize)]
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
pub struct PostData {
    title: String,
    body: String,
}

impl PostData {
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
