use crate::data::Id;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Id,
    pub username: String,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub email: String,
    pub registration_date: String,
    pub is_verified: bool,
    pub avatar: Option<String>,
}
