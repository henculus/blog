use argon2rs::verifier::Encoded;
use diesel::prelude::*;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

use crate::{Result, ViewError};
use crate::schema::users;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[primary_key(username)]
#[table_name = "users"]
pub struct User {
    username: String,
    password_hash: String,
    user_roles: Vec<String>,
}

impl User {
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn verify_password_and_generate_jwt(&self, password: String) -> Result<String> {
        unimplemented!()
    }
    pub fn from_jwt(jwt: String) -> Result<User> {
        unimplemented!()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ViewError;

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ViewError> {
        unimplemented!()
    }
}

impl From<UserData> for User {
    fn from(user_data: UserData) -> User {
        Self {
            username: user_data.username,
            password_hash: user_data.password.to_password_hash(),
            user_roles: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

trait PasswordHash {
    fn to_password_hash(&self) -> String;
    fn is_password_hash_correct(&self, raw_password: &String) -> bool;
}

impl PasswordHash for String {
    fn to_password_hash(&self) -> String {
        let salt: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        let data_hash = Encoded::default2i(
            self.as_bytes(),
            salt.as_bytes(),
            b"",
            b"").to_u8();
        String::from_utf8(data_hash).unwrap()
    }

    fn is_password_hash_correct(&self, raw_password: &String) -> bool {
        let hash = Encoded::from_u8(self.as_ref()).unwrap();
        hash.verify(raw_password.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::user::{PasswordHash, User, UserData};

    #[test]
    fn test_password_hashing() {
        let password = "password".to_string();
        let hash = password.to_password_hash();
        assert!(hash.is_password_hash_correct(&password))
    }

    #[test]
    fn test_wrong_password() {
        let password = "password".to_string();
        let hash = password.to_password_hash();
        let wrong_password = "wrong_password".to_string();
        assert!(!hash.is_password_hash_correct(&wrong_password))
    }

    #[test]
    fn test_user_from_user_data() {
        let username = &"Hello".to_string();
        let password = &"Hello".to_string();
        let user_data = UserData { username: username.to_string(), password: password.to_string() };

        let user: User = user_data.into();
        assert_eq!(user.username(), username);
        assert!(user.password_hash.is_password_hash_correct(password));
    }
}