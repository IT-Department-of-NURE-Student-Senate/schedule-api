#![allow(dead_code)]
mod db;
mod error;
mod fetcher;
mod models;
mod entities;

use error::Error;
use schedule_migrator::Migrator;
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    dotenvy::dotenv().ok();

    let db = db::get_connection().await?;

    let _schema_manager = SchemaManager::new(&db);

    Migrator::refresh(&db).await?;

    Ok(())
}
