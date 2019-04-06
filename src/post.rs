use serde::{Deserialize, Serialize};

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