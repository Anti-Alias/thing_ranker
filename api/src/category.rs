use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::{PgConnection, PgTransaction};

use crate::app::{ApiError, ApiResponse, AppState};

// TODO: Remove
const ROOT_ACCOUNT_ID: i32 = 1;

#[skip_serializing_none]
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub image: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(TryFromMultipart, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub file: FieldData<Bytes>,
}

pub async fn get_category(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> ApiResponse<Category> {
    let query: &str = "
        SELECT id,account_id,name,image,created,modified
        FROM category
        WHERE id = $1
    ";
    let category: Option<Category> = sqlx::query_as(query)
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(ApiError::CategoryNotFound);
    };
    Ok((StatusCode::OK, Json(category)))
}

pub async fn create_category(
    State(state): State<AppState>,
    TypedMultipart(request): TypedMultipart<CreateCategoryRequest>,
) -> ApiResponse<Category> {
    // Transaction start
    let mut tx: PgTransaction = state.pool.begin().await?;
    let conn = &mut *tx;
    let category_name = uuid::Uuid::new_v4().to_string();
    let category_name = format!("{category_name}.png");

    // Insert category
    if category_exists(&request.name, conn).await? {
        return Err(ApiError::CategoryAlreadyExists);
    }
    let query = "
        INSERT INTO category (account_id, name, image)
        VALUES ($1, $2, $3)
        RETURNING id,account_id,name,image,created,modified
    ";
    let category: Category = sqlx::query_as(query)
        .bind(ROOT_ACCOUNT_ID)
        .bind(&request.name)
        .bind(&category_name)
        .fetch_one(conn)
        .await?;
    // Transaction end
    tx.commit().await?;
    Ok((StatusCode::OK, Json(category)))
}

async fn category_exists(name: &str, conn: &mut PgConnection) -> Result<bool, ApiError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM category WHERE name=$1")
        .bind(name)
        .fetch_one(conn)
        .await?;
    Ok(count >= 1)
}
