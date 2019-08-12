use diesel::prelude::*;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket_contrib::json::Json;

use crate::{Database, Id};
use crate::error::Error;
use crate::schema::users::dsl::*;
use crate::user::{Token, User, UserData};
use crate::ViewResult;

const OFFSET: i64 = 0;
const LIMIT: i64 = 10;

type Username = String;

#[get("/users?<limit>&<offset>")]
pub fn get_users(conn: Database, _token: Token, limit: Option<i64>, offset: Option<i64>) -> ViewResult<Vec<User>> {
    let all_users = users
        .offset(offset.unwrap_or(OFFSET))
        .limit(limit.unwrap_or(LIMIT))
        .load::<User>(&*conn)?;

    Ok(Json(all_users))
}

#[post("/users", format = "json", data = "<user_data>")]
pub fn new_user(user_data: Json<UserData>, conn: Database) -> ViewResult<User> {
    let user: User = user_data.into_inner().into();
    let query_result = diesel::insert_into(users).values(user).get_result(&*conn)?;

    Ok(Json(query_result))
}

#[patch("/users/<updated_username>", format = "application/json", data = "<user_data>")]
pub fn update_user(
    updated_username: Username,
    user_data: Json<UserData>,
    conn: Database,
    token: Token,
    mut cookies: Cookies,
) -> ViewResult<User> {
    let updated_user_data: User = user_data.into_inner().into();
    let current_user = users.filter(username.eq(&updated_username));
    let query_result = diesel::update(current_user)
        .set(updated_user_data)
        .get_result(&*conn)?;

    cookies.remove_private(
        Cookie::named("token")
    );

    Ok(Json(query_result))
}

#[delete("/users")]
pub fn delete_user(conn: Database, token: Token) -> ViewResult<usize> {
    let user = users.filter(username.eq(token.username()));
    let query_result = diesel::delete(user).execute(&*conn)?;
    Ok(Json(query_result))
}

#[get("/session")]
pub fn get_session_info(token: Token) -> ViewResult<Token> {
    Ok(Json(token))
}

#[post("/session", format = "json", data = "<user_data>")]
pub fn login(user_data: Json<UserData>, conn: Database, mut cookies: Cookies) -> Result<(), Error> {
    let user_data = user_data.into_inner();
    let user: User = users.find(user_data.username).first::<User>(&*conn)?;
    let token = user.verify_password_and_generate_jwt(user_data.password)?;

    cookies.add_private(
        Cookie::build("token", token)
            .secure(false)
            .same_site(SameSite::None)
            .finish()
    );

    Ok(())
}

#[delete("/session")]
pub fn logout(_token: Token, mut cookies: Cookies) -> Result<(), Error> {
    cookies.remove_private(
        Cookie::named("token")
    );

    Ok(())
}
