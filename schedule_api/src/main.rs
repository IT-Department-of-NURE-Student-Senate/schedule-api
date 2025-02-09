#![allow(dead_code)]
mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    Ok(())
}
