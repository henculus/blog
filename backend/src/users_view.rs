use diesel::prelude::*;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket_contrib::json::Json;

use crate::{Database, Id};
use crate::error::Error;
use crate::schema::users::dsl::*;
use crate::user::{Token, User, UserData};
use crate::ViewResult;

#[get("/users")]
pub fn get_users(conn: Database) -> ViewResult<Vec<User>> {
    unimplemented!()
}

#[post("/users", format = "json", data = "<user_data>")]
pub fn new_user(user_data: Json<UserData>, conn: Database) -> ViewResult<User> {
    let user: User = user_data.into_inner().into();
    let query_result = diesel::insert_into(users).values(user).get_result(&*conn)?;

    Ok(Json(query_result))
}

#[patch("/users/<user_id>", format = "json", data = "<user_data>")]
pub fn update_user(
    user_id: Id,
    user_data: Json<UserData>,
    conn: Database,
    token: Token,
) -> ViewResult<String> {
    unimplemented!()
}

#[delete("/users/<user_id>")]
pub fn delete_user(user_id: Id, conn: Database, token: Token) -> ViewResult<String> {
    unimplemented!()
}

#[get("/session")]
pub fn get_session_info(token: Token, conn: Database) -> ViewResult<Token> {
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
pub fn logout(token: Token, mut cookies: Cookies) -> Result<(), Error> {
    cookies.remove_private(
        Cookie::named("token")
    );

    Ok(())
}
