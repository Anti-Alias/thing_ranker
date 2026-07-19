use axum::{
    extract::multipart::MultipartError,
    http::{StatusCode, header::ToStrError},
    response::{IntoResponse, Response},
};
use jwks_client_rs::JwksClientError;
use thiserror::Error;

use crate::{asset::AssetStoreError, image::ImageProcessingError};

/// Encompasses all errors that can occur in the API
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Thing not found")]
    ThingNotFound,
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Thing already exists")]
    ThingAlreadyExists,
    #[error("Rank already exists")]
    RankAlreadyExists,
    #[error("Query string must be at least 3 characters")]
    QueryStringTooSmall,
    #[error("Auth header missing")]
    AuthHeaderMissing,
    #[error("Auth header not a valid string: {0}")]
    AuthHeaderNotAString(#[from] ToStrError),
    #[error("Auth header missing bearer")]
    AuthHeaderMissingBearer,
    #[error("Auth header missing JWT")]
    AuthHeaderMissingJWT,
    #[error("Auth header malformed")]
    AuthHeaderMalformed,
    #[error("Auth header missing kid")]
    AuthHeaderMissingKid,
    #[error("Auth header decoding failed: {0}")]
    AuthHeaderDecodingFailed(#[from] jsonwebtoken::errors::Error),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Base64 decoding failed")]
    Base64DecodingFailed,
    #[error("Jwks client failed to decode JWT: {0}")]
    JwksClientError(#[from] JwksClientError),
    #[error(transparent)]
    ImageProcessingError(#[from] ImageProcessingError),
    #[error("Asset store error: {0}")]
    AssetStoreError(#[from] AssetStoreError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    MultipartError(#[from] MultipartError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            Self::CategoryNotFound => StatusCode::NOT_FOUND,
            Self::ThingNotFound => StatusCode::NOT_FOUND,
            Self::CategoryAlreadyExists => StatusCode::BAD_REQUEST,
            Self::ThingAlreadyExists => StatusCode::BAD_REQUEST,
            Self::RankAlreadyExists => StatusCode::BAD_REQUEST,
            Self::QueryStringTooSmall => StatusCode::BAD_REQUEST,
            Self::AuthHeaderMissing => StatusCode::BAD_REQUEST,
            Self::AuthHeaderNotAString(_) => StatusCode::BAD_REQUEST,
            Self::AuthHeaderMissingBearer => StatusCode::BAD_REQUEST,
            Self::AuthHeaderMissingJWT => StatusCode::BAD_REQUEST,
            Self::AuthHeaderMalformed => StatusCode::BAD_REQUEST,
            Self::AuthHeaderMissingKid => StatusCode::BAD_REQUEST,
            Self::AuthHeaderDecodingFailed(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Base64DecodingFailed => StatusCode::BAD_REQUEST,
            Self::JwksClientError(_) => StatusCode::BAD_REQUEST,
            Self::ImageProcessingError(_) => StatusCode::BAD_REQUEST,
            Self::AssetStoreError(error) => {
                log::error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::SqlxError(error) => {
                log::error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::MultipartError(error) => {
                log::error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let message = match status_code.is_server_error() {
            true => String::from("Internal Server Error"),
            false => self.to_string(),
        };
        (status_code, message).into_response()
    }
}
