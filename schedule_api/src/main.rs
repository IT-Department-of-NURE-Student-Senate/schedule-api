#![allow(dead_code)]
mod db;
mod entities;
mod error;
mod fetcher;
mod models;

use error::Error;
use fetcher::Fetcher;
// use schedule_migrator::Migrator;
// use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    dotenvy::dotenv().ok();

    let repository = db::Repository::new().await?;

    repository.run_migrations().await?;

    // let fetcher = fetcher::WebFetcher::new().await.unwrap();

    let fetcher = fetcher::FileFetcher::new();

    let fetched = fetcher.fetch_podr().await?;

    repository.update_from_podr(fetched).await?;

    Ok(())
}
