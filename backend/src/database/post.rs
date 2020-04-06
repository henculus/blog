use crate::data::{Id, NewPostInfo, Post, UpdatePostInfo};
use crate::database::error::Error;
use crate::database::Connection;
use futures::{Future, FutureExt};
use log::*;
use std::convert::{TryFrom, TryInto};
use tokio_postgres::Row;

pub fn retrieve_post(
    id: Id,
) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<Post, Error>> + Unpin)> {
    move |conn| Box::new(retrieve_post_query(conn, id).boxed())
}

pub fn retrieve_posts(
    page: i32,
    page_size: i32,
) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<Vec<Post>, Error>> + Unpin)> {
    move |conn| Box::new(retrieve_posts_query(conn, page, page_size).boxed())
}

pub fn change_post(
    id: Id,
    post: UpdatePostInfo,
) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<(), Error>> + Unpin)> {
    move |conn| Box::new(update_post_query(conn, id, post.clone()).boxed())
}

pub fn remove_post(
    id: Id,
) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<(), Error>> + Unpin)> {
    move |conn| Box::new(remove_post_query(conn, id).boxed())
}

pub fn create_post(
    post: NewPostInfo,
) -> impl Fn(Connection) -> Box<(dyn Future<Output = Result<Id, Error>> + Unpin)> {
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
        .query_one("SELECT ID, TITLE, BODY, CREATED_AT, EDITED_AT, AUTHOR, IS_PUBLISHED FROM POSTS WHERE ID=$1::INT", &[&id.0])
        .await?;

    let title: String = row.get(1);

    debug!("{:#?}", title);

    let post = row.try_into()?;
    Ok(post)
}

async fn retrieve_posts_query(
    connection: Connection,
    page: i32,
    page_size: i32,
) -> Result<Vec<Post>, Error> {
    let limit = page_size;
    let offset = page_size * (page - 1);

    let rows = connection
        .query(
            "SELECT ID, TITLE, BODY, CREATED_AT, EDITED_AT, AUTHOR, IS_PUBLISHED FROM POSTS LIMIT $1::INT OFFSET $2::INT",
            &[&limit, &offset],
        )
        .await?;

    rows.into_iter().map(|row| row.try_into()).collect()
}

impl TryFrom<Row> for Post {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Post {
            id: Id(value.get(0)),
            title: value.get(1),
            body: value.get(2),
            created_at: value.get(3),
            edited_at: value.get(4),
            author: Id(value.get(5)),
            is_published: value.get(6),
        })
    }
}
