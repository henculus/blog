use std::env;

use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use crate::models::error::{ModelError, ModelErrorKind};

pub mod error;
pub mod post;
pub mod schema;

type DBPool = Pool<ConnectionManager<PgConnection>>;
type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type Id = i32;

pub trait Model {
    type Item;
    type NewItem;
    fn new(connection: DBConnection) -> Self;
    fn create(&self, item: Self::NewItem) -> Result<Self::Item, ModelError>;
    fn update(&self, item_id: Id, item: Self::NewItem) -> Result<Id, ModelError>;
    fn get(&self, limit: i64, offset: i64) -> Result<Vec<Self::Item>, ModelError>;
    fn get_by_id(&self, item_id: Id) -> Result<Self::Item, ModelError>;
    fn delete(&self, item_id: Id) -> Result<Id, ModelError>;
}

pub struct TableManager {
    connection_pool: DBPool,
}

impl TableManager {
    pub fn new(db_pool: DBPool) -> Self {
        Self {
            connection_pool: db_pool,
        }
    }
    // TODO: May be there is an error: request while connection pool is fully busy response will be
    //  an error instead of waiting for free connection. Or may be it is not an error
    pub fn get<T: Model>(&self) -> Result<T, ModelError> {
        match self.connection_pool.try_get() {
            Some(conn) => Ok(T::new(conn)),
            None => Err(ModelError {
                kind: ModelErrorKind::DBConnectionError,
                message: "Cannot get connection from pool".to_string(),
            }),
        }
    }
}

pub fn db_pool() -> Result<DBPool, ModelError> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Ok(Pool::new(manager)?)
}
