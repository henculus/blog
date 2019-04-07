use std::convert::TryFrom;

use argon2rs::verifier::Encoded;
use jsonwebtoken::{Algorithm, decode, encode, Header, Validation};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{Error, ViewResult};
use crate::schema::users;

const SECRET_KEY: &str = "l5KtZcWen4XT4F77Dg2shixUzaIqdWohQf9MEbnjBi0=";
const JWT_K: &[u8] = b"";
const JWT_X: &[u8] = b"";
const HASH_SALT_LEN: usize = 10;
const JWT_EXP_TIME: i64 = 86400;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[primary_key(username)]
#[table_name = "users"]
pub struct User {
    username: String,
    password_hash: String,
    user_roles: Vec<String>,
}

impl User {
    pub fn verify_password_and_generate_jwt(&self, password: String) -> ViewResult<String> {
        if !self.password_hash.is_password_hash_correct(&password) {
            return Err(Error::WrongPassword);
        }
        let now = time::get_time().sec;
        let payload = Token {
            iat: now,
            exp: now + JWT_EXP_TIME,
            sub: self.username.to_string(),
        };

        let token = encode(&Header::default(),
                           &payload,
                           SECRET_KEY.as_ref())?;
        Ok(Json(token))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    iat: i64,
    exp: i64,
    pub sub: String,
}

impl Token {
    pub fn username(&self) -> &String {
        &self.sub
    }
}

impl TryFrom<String> for Token {
    type Error = Error;

    fn try_from(raw_token: String) -> Result<Token, Self::Error> {
        let data = decode::<Token>(
            &raw_token,
            SECRET_KEY.as_ref(),
            &Validation::new(Algorithm::HS256))?;
        let token = data.claims;

        Ok(token)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Token, Self::Error> {
        let header = match request.headers().get_one("Authorization") {
            Some(h) => h,
            None => return Outcome::Failure((Status::Unauthorized, Error::NoAuthHeader))
        };
        let header_parts: Vec<&str> = header.split(" ").collect();
        if header_parts[0] != "Bearer" {
            return Outcome::Failure((Status::Unauthorized, Error::WrongAuthType));
        }

        let token = header_parts[1];
        match Token::try_from(token.to_string()) {
            Ok(t) => Outcome::Success(t),
            Err(e) => Outcome::Failure((Status::Unauthorized, e))
        }
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
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(HASH_SALT_LEN)
            .collect();

        let data_hash = Encoded::default2i(
            self.as_bytes(),
            salt.as_bytes(),
            JWT_K,
            JWT_X).to_u8();
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