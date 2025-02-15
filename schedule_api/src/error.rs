use crate::db;
use crate::fetcher;
use crate::models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Model(#[from] models::Error),

    #[error(transparent)]
    Fetcher(#[from] fetcher::Error),

    #[error(transparent)]
    Db(#[from] db::Error),

    #[error(transparent)]
    SeaOrm(#[from] sea_orm::DbErr),
}
