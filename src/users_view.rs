use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::Database;
use crate::Result;
use crate::schema::users::dsl::*;
use crate::user::{User, UserData};

#[post("/users", format = "json", data = "<user_data>")]
pub fn new_user(user_data: Json<UserData>, conn: Database) -> Result<User> {
    let user: User = user_data.into_inner().into();
    let query_result = diesel::insert_into(users)
        .values(user)
        .get_result(&*conn)?;

    Ok(Json(query_result))
}

#[post("/users/login", format = "json", data = "<user_data>")]
pub fn login(user_data: Json<UserData>, conn: Database) -> Result<String> {
    let user_data = user_data.into_inner();
    let user: User = users.find(user_data.username).first::<User>(&*conn)?;
    Ok(user.verify_password_and_generate_jwt(user_data.password)?)
}