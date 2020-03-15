use crate::data::{Id, Post};
use crate::database::error::Error;
use crate::database::Connection;

pub async fn create_post(connection: Connection, post: Post) -> Result<Id, Error> {
    unimplemented!()
}

pub async fn update_post(connection: Connection, post: Post) -> Result<Id, Error> {
    unimplemented!()
}

pub async fn remove_post(connection: Connection, id: Id) -> Result<(), Error> {
    unimplemented!()
}

pub async fn retrieve_post(connection: Connection, id: Id) -> Result<Post, Error> {
    unimplemented!()
}

pub async fn retrieve_posts(
    connection: Connection,
    page: i32,
    page_size: i32,
) -> Result<Vec<Post>, Error> {
    unimplemented!()
}
