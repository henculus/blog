use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::{Json, JsonError};
use serde::{Deserialize, Serialize};

use crate::{
    DBConn,
    models::{Model, user::*},
    views::error::*,
};

#[post("/users", format = "json", data = "<user>")]
pub fn new_user(
    user: Result<Json<NewUser>, JsonError>,
    conn: DBConn,
) -> Result<Json<String>, ViewError> {
    let user = user?;
    let users = UsersTable(&*conn);

    users
        .create(user.into_inner())
        .map(|user| Ok(Json(user.username)))?
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[post("/users/login", format = "json", data = "<login_form>")]
pub fn login(
    mut cookies: Cookies,
    login_form: Result<Json<LoginForm>, JsonError>,
    conn: DBConn,
) -> Result<(), ViewError> {
    let user_data = login_form?.into_inner();
    let users = UsersTable(&*conn);

    let user = users.get_by_id(user_data.username)?;
    user
        .check_password(user_data.password)
        .map(
            |_| {
                cookies.add_private(Cookie::new("logged_in", true.to_string()));
                Ok(())
            }
        )?
}
