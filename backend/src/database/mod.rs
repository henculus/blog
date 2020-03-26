use deadpool_postgres::{Client, Pool};
use futures::Future;

pub use error::*;
pub use post::*;
pub use user::*;

pub mod error;
pub mod post;
pub mod user;

pub type Connection = Client;

#[derive(Clone)]
pub struct DatabaseClient {
    pool: Pool,
}

impl DatabaseClient {
    pub async fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn execute<T, F>(&self, f: F) -> Result<T, Error>
    where
        F: Fn(Client) -> Box<(dyn Future<Output = Result<T, Error>> + Unpin)>,
    {
        let conn = self.pool.get().await?;
        f(conn).await
    }
}
