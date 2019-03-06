pub mod post;
pub mod schema;
pub mod errors;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use crate::models::errors::ModelError;

pub type Id = i32;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;


pub fn db_pool() -> Result<DBPool, ModelError> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Ok(Pool::new(manager)?)
}