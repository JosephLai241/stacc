//! Contains all errors that may be raised in `stacc`.

use std::env::VarError;

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{self, Display};
use thiserror::Error;

use crate::models::data::Response;

/// Contains all error variants for errors that may be raised by Actix Web endpoints.
#[derive(Debug, Display, derive_more::Error)]
pub enum StaccResponseError {
    /// A generic error variant for MongoDB.
    #[display(fmt = "MongoDB error: {error}")]
    MongoDBError { error: String },

    /// Could not find a document within MongoDB.
    #[display(fmt = "MongoDB search error: {error}")]
    MongoDBSearchError { error: String },
}

impl ResponseError for StaccResponseError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(Response {
                message: self.to_string(),
                status_code: self.status_code().as_u16(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            StaccResponseError::MongoDBError { .. } => {
                StatusCode::from_u16(500).unwrap_or(StatusCode::BAD_REQUEST)
            }
            StaccResponseError::MongoDBSearchError { .. } => StatusCode::NOT_FOUND,
        }
    }
}

/// Contains all error variants for errors that may be raised by functions that do not return an
/// `HttpResponse` (functions that are not Actix Web endpoints).
#[derive(Debug, Error)]
pub enum StaccError {
    /// Something fucked up while retrieving an environment variable.
    #[error("Environment error: {0}")]
    EnvironmentError(#[from] VarError),

    /// Something fucked up with MongoDB.
    #[error("MongoDB error: {0}")]
    MongoDBError(#[from] mongodb::error::Error),

    /// Something fucked up while making a request with `reqwest`.
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
