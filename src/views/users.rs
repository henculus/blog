use rocket::http::{Cookie, Cookies, Status};
use rocket::outcome::Outcome::{Failure, Forward, Success};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket_contrib::json::{Json, JsonError};
use serde::{Deserialize, Serialize};

use crate::{
    DBConn,
    models::{user::*},
    views::error::*,
};

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ViewError;

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ViewError> {
        let payload = request.cookies()
            .get("payload")
            .map(|cookie| format!("{}", cookie.value()));
        let sign = request.cookies()
            .get_private("sign")
            .map(|cookie| format!("{}", cookie.value()));

        println!("payload: {:?}, sign: {:?}", payload, sign);

        let token = match (payload, sign) {
            (Some(p), Some(s)) => format!("{}{}", p, s),
            _ => return Failure((Status::Unauthorized, ViewError::new_unauthorized()))
        };

        let user = User::from_jwt(token);

        match user {
            Ok(user) => Success(user),
            Err(_) => Failure((Status::Unauthorized, ViewError::new_unauthorized()))
        }
    }
}

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
) -> Result<Json<String>, ViewError> {
    let user_data = login_form?.into_inner();
    let users = UsersTable(&*conn);

    let user = users.get_by_id(user_data.username)?;
    user
        .check_password_and_generate_jwt(user_data.password)
        .map(
            |token| {
                let split_pos = token.rfind('.').unwrap_or(0);
                let (payload, sign) = token.split_at(split_pos);
                let mut sign_cookie = Cookie::new("sign", sign.to_string());
                let mut payload_cookie = Cookie::new("payload", payload.to_string());
                sign_cookie.set_http_only(true);
                payload_cookie.set_http_only(false);
                payload_cookie.set_path("/");
                cookies.add_private(sign_cookie);
                cookies.add(payload_cookie);
                Ok(Json(payload.to_string()))
            }
        )?
}
