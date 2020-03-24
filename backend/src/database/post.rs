use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::database::error::Error;
use crate::database::Connection;
use futures::Future;

type PostFuture = impl Future<Output = Result<Post, Error>>;
type PostQuery = impl Fn(Connection) -> PostFuture;

pub fn retrieve_post(id: Id) -> PostQuery {
    Box::new(move |conn| retrieve_post_query(conn, id))
}

async fn create_post_query(connection: Connection, post: NewPostInfo) -> Result<Id, Error> {
    unimplemented!()
}

async fn update_post_query(
    connection: Connection,
    id: Id,
    post: UpdatePostInfo,
) -> Result<(), Error> {
    unimplemented!()
}

async fn remove_post_query(connection: Connection, id: Id) -> Result<(), Error> {
    unimplemented!()
}

async fn retrieve_post_query(connection: Connection, id: Id) -> Result<Post, Error> {
    let client = connection
        .query_one("SELECT $1::INT FROM POSTS", &[&id.0])
        .await?;
    unimplemented!();
}

async fn retrieve_posts_query(
    connection: Connection,
    page: i32,
    page_size: i32,
) -> Result<Vec<Post>, Error> {
    unimplemented!()
}
