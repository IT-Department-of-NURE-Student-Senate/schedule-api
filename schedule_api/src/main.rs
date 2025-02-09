#![allow(dead_code)]
mod error;
mod models;

use error::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    Ok(())
}
