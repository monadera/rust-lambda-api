use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("item {0} was not found")]
    NotFound(String),
    #[error("unknown error")]
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;
