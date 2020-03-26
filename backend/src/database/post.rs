use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::database::error::Error;
use crate::database::Connection;
use futures::{Future, FutureExt};
use std::convert::{TryInto, TryFrom};
use tokio_postgres::Row;

pub fn retrieve_post(id: Id) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<Post, Error>> + Unpin)>
{
    move |conn| Box::new(retrieve_post_query(conn, id).boxed())
}

pub fn retrieve_posts(page: i32, page_size: i32) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<Vec<Post>, Error>> + Unpin)> {
    move |conn| Box::new(retrieve_posts_query(conn, page, page_size).boxed())
}

pub fn update_post(id: Id, post: UpdatePostInfo) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<(), Error>> + Unpin)> {
    move |conn| Box::new(update_post_query(conn, id, post.clone()).boxed())
}

pub fn remove_post(id: Id) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<(), Error>> + Unpin)> {
    move |conn| Box::new(remove_post_query(conn, id).boxed())
}

pub fn create_post(post: NewPostInfo) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<Id, Error>> + Unpin)> {
    move |conn| Box::new(create_post_query(conn, post.clone()).boxed())
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
    let row = connection
        .query_one("SELECT $1::INT FROM POSTS", &[&id.0])
        .await?;
    let post = row.try_into()?;
    Ok(post)
}

async fn retrieve_posts_query(
    connection: Connection,
    page: i32,
    page_size: i32,
) -> Result<Vec<Post>, Error> {
    unimplemented!()
}


impl TryFrom<Row> for Post {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}