use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{DBConnection, error::*, Model, schema::users, user::hasher::HashablePassword};

mod hasher;

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Serialize, Deserialize)]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub user_roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub user_roles: Vec<String>,
}

impl User {
    pub fn generate_jwt(&self, password: String) -> Result<String, ModelError> {
        unimplemented!()
    }

    pub fn verify_jwt(&self) -> String {
        unimplemented!()
    }
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

struct UserToken {
    iat: i64,
    exp: i64,
    user: String,
    roles: Vec<String>,
}

pub struct UsersTable {
    db_connection: DBConnection
}

impl Model for UsersTable {
    type Key = String;
    type Item = User;
    type NewItem = NewUser;

    fn new(connection: DBConnection) -> Self {
        Self {
            db_connection: connection
        }
    }

    fn create(&self, new_user: NewUser) -> Result<User, ModelError> {
        let user: User = new_user.into();

        let result = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&*self.db_connection)?;
        Ok(result)
    }

    fn update(&self, item_id: String, item: NewUser) -> Result<i32, ModelError> {
        unimplemented!()
    }

    fn get(&self, limit: i64, offset: i64) -> Result<Vec<User>, ModelError> {
        unimplemented!()
    }

    fn get_by_id(&self, item_id: String) -> Result<User, ModelError> {
        unimplemented!()
    }

    fn delete(&self, item_id: String) -> Result<i32, ModelError> {
        unimplemented!()
    }
}