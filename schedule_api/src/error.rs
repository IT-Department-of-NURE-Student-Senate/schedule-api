use crate::fetcher;
use crate::models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ModelError(#[from] models::Error),

    #[error(transparent)]
    FetcherError(#[from] fetcher::Error),
}
