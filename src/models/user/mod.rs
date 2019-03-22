extern crate time;

use diesel::prelude::*;
use jsonwebtoken::{Algorithm, decode, encode, Header, Validation};
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

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    iat: i64,
    exp: i64,
    sub: String,
}

impl User {
    fn check_password(&self, password: String) -> Result<(), ModelError> {
        self.password_hash.verify_hash(&password)
    }

    pub fn check_password_and_generate_jwt(&self, password: String) -> Result<String, ModelError> {
        self.check_password(password)?;
        let now = time::get_time().sec;
        let payload = UserToken {
            iat: now,
            exp: now + 86400,
            sub: "".to_string(),
        };


        //TODO: Rewrite this
        // im too lazy for now for adding one more ModelError case
        // anyway all errors system should be rewrited fully
        encode(&Header::default(),
               &payload,
               "l5KtZcWen4XT4F77Dg2shixUzaIqdWohQf9MEbnjBi0=".as_ref())
            .map_err(|_| ModelError {
                kind: ModelErrorKind::InvalidCredentials,
                message: "Error while processing login request".to_string(),
            })
    }

    pub fn verify_jwt(&self, token: String) -> Result<(), ModelError> {
        decode::<UserToken>(&token,
                            "l5KtZcWen4XT4F77Dg2shixUzaIqdWohQf9MEbnjBi0=".as_ref(),
                            &Validation::new(Algorithm::HS256))
            .map(|data| ())
            .map_err(|_|
                ModelError {
                    kind: ModelErrorKind::InvalidCredentials,
                    message: "Error while processing JWT token".to_string(),
                }
            )
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


mod tests {
    use crate::models::user::{NewUser, User};

    #[test]
    fn test_check_jwt() {
        let password = "qwerty".to_string();
        let user_data = NewUser {
            username: "test".to_string(),
            password: password.clone(),
            user_roles: vec![],
        };

        let user: User = user_data.into();

        let token = user.check_password_and_generate_jwt(password.clone()).unwrap();
        println!("{}", token);
        assert!(user.verify_jwt(token).is_ok());
    }
}