use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{error::*, Model, schema::users, user::hasher::HashablePassword};

mod hasher;

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Serialize, Deserialize)]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub user_roles: Vec<String>,
}

impl User {
    pub fn check_password(&self, password: String) -> Result<(), ModelError> {
        self.password_hash.verify_hash(&password)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub user_roles: Vec<String>,
}

impl From<NewUser> for User {
    fn from(new_user: NewUser) -> Self {
        Self {
            username: new_user.username,
            password_hash: new_user.password.hash(),
            user_roles: new_user.user_roles,
        }
    }
}

pub struct UsersTable<'a>(pub &'a PgConnection);

impl<'a> Model for UsersTable<'a> {
    type Key = String;
    type Item = User;
    type NewItem = NewUser;

    fn create(&self, new_user: NewUser) -> Result<User, ModelError> {
        let user: User = new_user.into();

        let result = diesel::insert_into(users::table)
            .values(&user)
            .get_result(self.0)?;
        Ok(result)
    }

    fn update(&self, item_id: String, item: NewUser) -> Result<i32, ModelError> {
        unimplemented!()
    }

    fn get(&self, limit: i64, offset: i64) -> Result<Vec<User>, ModelError> {
        unimplemented!()
    }

    fn get_by_id(&self, username: String) -> Result<User, ModelError> {
        let result = users::table.find(username).first::<User>(self.0)?;
        Ok(result)
    }

    fn delete(&self, username: String) -> Result<i32, ModelError> {
        unimplemented!()
    }
}
