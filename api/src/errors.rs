//! Contains all errors that may be raised in `stacc`.

use std::env::VarError;

use derive_more::{self, Display};
use thiserror::Error;

#[derive(Debug, Display, derive_more::Error)]
pub enum StaccResponseError {}

#[derive(Debug, Error)]
pub enum StaccError {
    /// Something fucked up while retrieving an environment variable.
    #[error("Environment error: {0}")]
    EnvironmentError(#[from] VarError),

    /// Something fucked up with MongoDB.
    #[error("MongoDB error: {0}")]
    MongoDBError(#[from] mongodb::error::Error),
}
