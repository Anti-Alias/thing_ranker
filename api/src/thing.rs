use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgTransaction};

use crate::app::{ApiError, ApiResponse, AppState};

// TODO: Remove
const ROOT_ACCOUNT_ID: i32 = 1;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Thing {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    pub created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
}

#[derive(TryFromMultipart, Debug)]
pub struct CreateThingRequest {
    pub name: String,
    pub file: FieldData<Bytes>,
}

pub async fn get_thing(Path(id): Path<i32>, State(state): State<AppState>) -> ApiResponse<Thing> {
    const QUERY: &str = "
        SELECT
            id,
            account_id,
            name,
            image_name,
            created,
            modified
        FROM
            thing
        WHERE
            id = $1
    ";
    let thing: Option<Thing> = sqlx::query_as(QUERY)
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(thing) = thing else {
        return Err(ApiError::ThingNotFound);
    };
    Ok((StatusCode::OK, Json(thing)))
}

pub async fn create_thing(
    State(state): State<AppState>,
    TypedMultipart(request): TypedMultipart<CreateThingRequest>,
) -> ApiResponse<Thing> {
    // Transaction start
    let mut tx: PgTransaction = state.pool.begin().await?;
    let conn = &mut *tx;
    let image_name = request
        .file
        .metadata
        .file_name
        .ok_or(ApiError::MissingFileName)?;

    // Insert thing
    if thing_exists(&request.name, conn).await? {
        return Err(ApiError::ThingAlreadyExists);
    }
    const QUERY: &str = "
        INSERT INTO thing (account_id, name, image_name)
        VALUES ($1, $2, $3)
        RETURNING id,account_id,name,image_name,created,modified
    ";
    let thing: Thing = sqlx::query_as(QUERY)
        .bind(ROOT_ACCOUNT_ID)
        .bind(&request.name)
        .bind(&image_name)
        .fetch_one(conn)
        .await?;
    // Transaction end
    tx.commit().await?;
    Ok((StatusCode::OK, Json(thing)))
}

async fn thing_exists(name: &str, conn: &mut PgConnection) -> Result<bool, ApiError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM thing WHERE name=$1")
        .bind(name)
        .fetch_one(conn)
        .await?;
    Ok(count >= 1)
}
