use crate::data::Id;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Id,
    username: String,
    first_name: Option<String>,
    second_name: Option<String>,
    email: String,
    registration_date: String,
    verified: bool,
    avatar: Option<String>,
}
