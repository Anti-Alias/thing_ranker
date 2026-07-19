use axum::{Json, extract::State};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};

use crate::app::{ApiError, ApiResponse, AppState};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Rank {
    pub thing_id: i32,
    pub category_id: i32,
    pub wins: i32,
    pub losses: i32,
    pub win_loss_ratio: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRankRequest {
    pub thing_id: i32,
    pub category_id: i32,
}

/// Adds a rank, which connects a thing to a category
pub async fn create_rank(
    State(state): State<AppState>,
    Json(request): Json<CreateRankRequest>,
) -> ApiResponse<Rank> {
    if !thing_exists(request.thing_id, &state.pool).await? {
        return Err(ApiError::ThingNotFound);
    }
    if !category_exists(request.category_id, &state.pool).await? {
        return Err(ApiError::CategoryNotFound);
    }
    if rank_exists(request.thing_id, request.category_id, &state.pool).await? {
        return Err(ApiError::RankAlreadyExists);
    }
    let query = "
        INSERT INTO rank (thing_id,category_id) VALUES ($1,$2)
        RETURNING thing_id,category_id,wins,losses,win_loss_ratio
    ";
    let rank: Rank = sqlx::query_as(query)
        .bind(request.thing_id)
        .bind(request.category_id)
        .fetch_one(&state.pool)
        .await?;
    Ok((StatusCode::CREATED, Json(rank)))
}

async fn thing_exists(thing_id: i32, pool: &PgPool) -> Result<bool, ApiError> {
    let query = "SELECT(EXISTS(SELECT * FROM thing WHERE id=$1 LIMIT 1))";
    let exists: bool = sqlx::query_scalar(query)
        .bind(thing_id)
        .fetch_one(pool)
        .await?;
    Ok(exists)
}

async fn category_exists(category_id: i32, pool: &PgPool) -> Result<bool, ApiError> {
    let query = "SELECT(EXISTS(SELECT * FROM category WHERE id=$1 LIMIT 1))";
    let exists: bool = sqlx::query_scalar(query)
        .bind(category_id)
        .fetch_one(pool)
        .await?;
    Ok(exists)
}

async fn rank_exists(thing_id: i32, category_id: i32, pool: &PgPool) -> Result<bool, ApiError> {
    let query = "SELECT(EXISTS(SELECT * FROM rank WHERE thing_id=$1 AND category_id=$2 LIMIT 1))";
    let exists: bool = sqlx::query_scalar(query)
        .bind(thing_id)
        .bind(category_id)
        .fetch_one(pool)
        .await?;
    Ok(exists)
}
