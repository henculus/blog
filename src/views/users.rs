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

fn get_token_from_cookies(request: &Request) -> Result<String, ViewError> {
    let payload = request.cookies()
        .get("payload")
        .map(|cookie| format!("{}", cookie.value()));
    let sign = request.cookies()
        .get_private("sign")
        .map(|cookie| format!("{}", cookie.value()));

    match (payload, sign) {
        (Some(p), Some(s)) => Ok(format!("{}{}", p, s)),
        _ => Err(ViewError::new_unauthorized())
    }
}

fn get_token_from_auth_header(request: &Request) -> Result<String, ViewError> {
    let header = request.headers()
        .get_one("Authorization")
        .ok_or(ViewError::new_unauthorized())?;
    let space_pos = header.find(' ').ok_or(ViewError::new_unauthorized())?;
    let (_, token) = header.split_at(space_pos + 1);
    Ok(token.to_string())
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ViewError;

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ViewError> {
        let token = get_token_from_cookies(request)
            .or(get_token_from_auth_header(request));

        let token = match token {
            Ok(t) => t,
            Err(e) => return Failure((Status::Unauthorized, e))
        };

        println!("{}", token);

        let user = User::from_jwt(token);
        println!("{:?}", user);
        match user {
            Ok(user) => Success(user),
            Err(err) => Failure((Status::Unauthorized, ViewError::new_unauthorized()))
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

#[post("/users/token", format = "json", data = "<login_form>")]
pub fn token(
    login_form: Result<Json<LoginForm>, JsonError>,
    conn: DBConn,
) -> Result<Json<String>, ViewError> {
    let user_data = login_form?.into_inner();
    let users = UsersTable(&*conn);

    let user = users.get_by_id(user_data.username)?;
    user
        .check_password_and_generate_jwt(user_data.password)
        .map(|token| Ok(Json(token)))?
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
