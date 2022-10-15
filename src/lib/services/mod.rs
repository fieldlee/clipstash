pub mod ask;
pub mod action;

use crate::{ClipError,DataError};

#[derive(Debug,thiserror::Error)]
pub enum ServiceError {
    #[error("clip error {0}")]
    Clip(#[from] ClipError),
    #[error("data error {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
    #[error("Perission not met {0}")]
    PerissionError(String),
}

impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
            DataError::Database(d) => match d {
                sqlx::Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other))
            }
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other)),
        }
    }
}