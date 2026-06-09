mod app;
mod config;
mod error;

pub use app::*;
use axum::{Json, http::StatusCode};
pub use config::*;
pub use error::*;

pub type ApiResponse<T> = Result<(StatusCode, Json<T>), ApiError>;
