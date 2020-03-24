use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::database::error::Error;
use crate::database::Connection;
use futures::Future;

type Fut = impl Future<Output = Result<Post, Error>>;

pub async fn retrieve_post(id: Id) -> Box<dyn Fn(Connection) -> Fut> {
    Box::new(async move |conn| retrieve_post_query(conn, id).await)
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

async fn retrieve_post_query(mut connection: Connection, id: Id) -> Result<Post, Error> {
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
