use std::env;

use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use crate::models::error::{ModelError, ModelErrorKind};

pub mod error;
pub mod post;
pub mod schema;
pub mod user;

pub type Id = i32;

pub trait Model {
    type Key;
    type Item;
    type NewItem;
    fn create(&self, item: Self::NewItem) -> Result<Self::Item, ModelError>;
    fn update(&self, item_id: Self::Key, item: Self::NewItem) -> Result<i32, ModelError>;
    fn get(&self, limit: i64, offset: i64) -> Result<Vec<Self::Item>, ModelError>;
    fn get_by_id(&self, item_id: Self::Key) -> Result<Self::Item, ModelError>;
    fn delete(&self, item_id: Self::Key) -> Result<i32, ModelError>;
}
