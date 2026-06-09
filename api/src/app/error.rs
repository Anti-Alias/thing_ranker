use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

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
    #[error("Missing file name")]
    MissingFileName,
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
            Self::MissingFileName => StatusCode::BAD_REQUEST,
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
