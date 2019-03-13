use rocket::State;
use rocket_contrib::json::{Json, JsonError};
use serde::{Deserialize, Serialize};

use crate::{
    models::{Model, TableManager, user::*},
    views::error::*,
};

#[post("/users", format = "json", data = "<user>")]
pub fn new_user(
    user: Result<Json<NewUser>, JsonError>,
    table: State<TableManager>,
) -> Result<Json<String>, ViewError> {
    let user = user?;
    let users: UsersTable = table.get()?;

    users
        .create(user.into_inner())
        .map(|user| Ok(Json(user.username)))?
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[post("/users/login", format = "json", data = "<user>")]
pub fn login(user: Result<Json<LoginForm>, JsonError>, table: State<TableManager>) -> Result<Json<String>, ViewError> {
    let user_data = user?.into_inner();
    let users: UsersTable = table.get()?;

    let user = users.get_by_id(user_data.username)?;
    Ok(Json(user.generate_jwt(user_data.password)?))
}