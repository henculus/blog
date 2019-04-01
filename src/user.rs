use crate::{Id, Result};

struct User {
    username: String,
    password_hash: String,
}

impl User {
    pub fn verify_password_and_generate_jwt(password: String) -> Result<String> {}
}

#[derive(Insertable)]
struct NewUser {
    username: String,
    password: String,
}
