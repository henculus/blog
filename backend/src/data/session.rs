use crate::data::Id;

pub struct Session {
    user_id: Id,
    expiration: String,
    created: String,
}
