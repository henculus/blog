use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{error::*, Id, schema::posts, schema::posts::dsl::*};
use crate::models::user::User;

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
    pub author: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

pub struct PostsTable<'a>(pub &'a PgConnection);

impl<'a> PostsTable<'a> {
    pub fn create(&self, post: NewPost, post_author: String) -> Result<Post, ModelError> {
        if post.title.trim() == "" || post.body.trim() == "" {
            return Err(ModelError::ValidationError(Some("Title or body is empty".to_string())));
        }
        let result = diesel::insert_into(posts::table)
            .values((&post, author.eq(&post_author)))
            .get_result(self.0)?;
        Ok(result)
    }

    pub fn update(&self, post_id: Id, post: NewPost) -> Result<Id, ModelError> {
        if post.title.trim() == "" || post.body.trim() == "" {
            return Err(ModelError::ValidationError(Some("Title or body is empty".to_string())));
        }
        let result = diesel::update(posts::table.find(post_id))
            .set(&post)
            .execute(self.0)?;
        Ok(result as Id)
    }

    pub fn get(&self, limit: i64, offset: i64) -> Result<Vec<Post>, ModelError> {
        let result = posts::table
            .offset(offset)
            .limit(limit)
            .load::<Post>(self.0)?;
        Ok(result)
    }

    pub fn get_by_id(&self, post_id: Id) -> Result<Post, ModelError> {
        let result = posts::table.find(post_id).first::<Post>(self.0)?;
        Ok(result)
    }

    pub fn delete(&self, post_id: Id) -> Result<i32, ModelError> {
        let result = diesel::delete(posts::table.find(post_id)).execute(self.0)?;
        Ok(result as i32)
    }

    pub fn is_user_author(&self, user: User, post_id: Id) -> Result<&Self, ModelError> {
        let post = self.get_by_id(post_id)?;
        match post.author {
            ref username if username == &user.username => Ok(self),
            _ => Err(ModelError::InvalidCredentials(None))
        }
    }
}

// Ok, let's try to write all problems in single list
// 1. Errors: it's gives not much info - i want to know what exactly i didn't found in DB
// 2. Auth - i still cannot figure out a scheme for it that will work. Im thinking that im should
//    start with views and interfaces, to fixate desirable behavior. It's getting worse when i
//    realise that i still didn't infer how to deal with sessions/jwt/cookies/localstorage and etc
//    may be i should first just do it with simple JWT Auth Bearer LocalStorage, just to get
//    something already working, and after improve it with adding XSS and CSRF protections
//
//    Decisions what need to make:
//    * JWT or not - yes
//    * LocalStorage full / Distributed -  LocalStorage(payload)+HTTP only(signature) / Cookie only
//    * Stateful (only username) / Stateless (with roles, and other information)
//     - if we're going to stateful - should we implement in memory storage for sessions, or just
//       make DB query on every protected request to retrieve user's data
//     - if we're going to stateful - how we should protect from token compromising (refresh token)
// 3. CORS
// 4. API beyond site - how to make it invisible for requests not from Vue FE
// 5. How to deal with possibility of making mobile app
//
// So, our plan for tomorrow:
//  - read more about JWT: take a look at RFC OAuth 2.0 and read
//    https://auth0.com/blog/refresh-tokens-what-are-they-and-when-to-use-them/
//  - make this decisions
//  - rework errors system to be more informative
//
