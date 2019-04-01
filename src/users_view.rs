use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::{Id, Result};
use crate::Database;
use crate::schema::users::dsl::*;

#[post("/users", format = "json", data = "<user_data>")]
pub fn new_user(user_data: Json<NewUser>, conn: Database) -> Result<User> {
    diesel::insert_into(users)
        .values(user_data.into_inner())
        .execute(&conn)?
}

#[post("/users/login", format = "json", data = "<user_data>")]
pub fn login(user_data: Json<NewUser>, conn: Database) -> Result<String> {
    let user_data = user_data.into_inner();
    let user = users.find(user.username);
    user.verify_password_and_generate_jwt(user_data.password)?
}