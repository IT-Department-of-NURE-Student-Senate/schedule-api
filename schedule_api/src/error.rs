use crate::models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ModelError(#[from] models::Error),
}
