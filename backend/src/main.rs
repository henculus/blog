#![feature(fixed_size_array)]
#![feature(async_closure)]
#![feature(type_alias_impl_trait)]

use std::io;

use actix_web::{App, HttpServer};
use deadpool_postgres::Config;
use tokio_postgres::NoTls;

use crate::database::DatabaseClient;
use api::*;

mod api;
mod data;
mod database;
mod error;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let mut config: Config = Default::default();
    config.user = Some("lupusanay".into());
    config.password = Some("qwerty".into());
    config.dbname = Some("blog".into());
    let pool = config.create_pool(NoTls).unwrap();
    let client = DatabaseClient::new(pool).await;

    HttpServer::new(move || App::new().app_data(client.clone()).service(get_post))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
