mod app;
mod config;
mod error;
mod state;

pub use app::*;
use axum::{Json, http::StatusCode};
pub use config::*;
pub use error::*;
pub use state::*;

pub type ApiResponse<T> = Result<(StatusCode, Json<T>), ApiError>;
