#![allow(dead_code)]
mod error;
mod fetcher;
mod models;

use error::Error;
use fetcher::Fetcher;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let fetcher = Fetcher::new("http://cist.nure.ua/ias/app/tt")?;
    let uni = fetcher.fetch_groups().await.unwrap();

    println!("{:#?}", uni);

    Ok(())
}
