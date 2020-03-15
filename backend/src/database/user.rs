use crate::data::{Id, User};
use crate::database::error::Error;
use crate::database::Connection;

pub async fn create_user(connection: Connection, user: User) -> Result<Id, Error> {
    unimplemented!()
}

pub async fn update_user(connection: Connection, user: User) -> Result<(), Error> {
    unimplemented!()
}

pub async fn remove_user(connection: Connection, id: Id) -> Result<(), Error> {
    unimplemented!()
}

pub async fn retrieve_user(connection: Connection, id: Id) -> Result<User, Error> {
    unimplemented!()
}

pub async fn retrieve_users(
    connection: Connection,
    page: i32,
    page_size: i32,
) -> Result<Vec<User>, Error> {
    unimplemented!()
}
