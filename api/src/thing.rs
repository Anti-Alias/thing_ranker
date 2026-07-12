use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query, State},
    http::StatusCode,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use base64::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::{PgConnection, PgTransaction};

use crate::{
    app::{ApiError, ApiResponse, AppState},
    util::Order,
};

// TODO: Remove
const ROOT_ACCOUNT_ID: i32 = 1;

const THING_PAGE_SIZE: i32 = 50;

#[skip_serializing_none]
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub image_name: Option<String>,
    pub created: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(TryFromMultipart, Debug)]
pub struct CreateThingRequest {
    pub name: String,
    pub file: FieldData<Bytes>,
}

#[derive(Deserialize, Debug)]
pub struct ThingQuery {
    order: Option<Order>,
    cursor: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ThingPage {
    pub things: Vec<Thing>,
    pub cursor: Option<String>,
}

pub async fn get_thing(Path(id): Path<i32>, State(state): State<AppState>) -> ApiResponse<Thing> {
    let query = "
        SELECT id,account_id,name,image_name,created,modified
        FROM thing
        WHERE id = $1
    ";
    let thing: Option<Thing> = sqlx::query_as(query)
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(thing) = thing else {
        return Err(ApiError::ThingNotFound);
    };
    Ok((StatusCode::OK, Json(thing)))
}

pub async fn get_thing_page(
    Query(params): Query<ThingQuery>,
    State(state): State<AppState>,
) -> ApiResponse<ThingPage> {
    // Gets a page of things
    let order = params.order.unwrap_or_default();
    let things: Vec<Thing> = match (params.cursor, order) {
        (Some(cursor), Order::Asc) => {
            let name = BASE64_STANDARD
                .decode(cursor)
                .map_err(|_| ApiError::Base64DecodingFailed)?;
            let name: &str = str::from_utf8(&name).map_err(|_| ApiError::Base64DecodingFailed)?;
            let query = "
                SELECT id,account_id,name,image_name,created,modified
                FROM thing
                WHERE name > $1
                ORDER BY name ASC
                LIMIT $2
            ";
            sqlx::query_as(query)
                .bind(name)
                .bind(THING_PAGE_SIZE)
                .fetch_all(&state.pool)
                .await?
        }
        (Some(cursor), Order::Desc) => {
            let name = BASE64_STANDARD
                .decode(cursor)
                .map_err(|_| ApiError::Base64DecodingFailed)?;
            let name: &str = str::from_utf8(&name).map_err(|_| ApiError::Base64DecodingFailed)?;
            let query = "
                SELECT id,account_id,name,image_name,created,modified
                FROM thing
                WHERE name < $1
                ORDER BY name DESC
                LIMIT $2
            ";
            sqlx::query_as(query)
                .bind(name)
                .bind(THING_PAGE_SIZE)
                .fetch_all(&state.pool)
                .await?
        }
        (None, Order::Asc) => {
            let query = "
                SELECT id,account_id,name,image_name,created,modified
                FROM thing
                ORDER BY name ASC
                LIMIT $1
            ";
            sqlx::query_as(query)
                .bind(THING_PAGE_SIZE)
                .fetch_all(&state.pool)
                .await?
        }
        (None, Order::Desc) => {
            let query = "
                SELECT id,account_id,name,image_name,created,modified
                FROM thing
                ORDER BY name DESC
                LIMIT $1
            ";
            sqlx::query_as(query)
                .bind(THING_PAGE_SIZE)
                .fetch_all(&state.pool)
                .await?
        }
    };
    // Creates cursor using last thing in page
    let cursor = things
        .last()
        .map(|thing| BASE64_STANDARD.encode(&thing.name));
    // Response
    let page = ThingPage { things, cursor };
    Ok((StatusCode::OK, Json(page)))
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
    let query = "
        INSERT INTO thing (account_id, name, image_name)
        VALUES ($1, $2, $3)
        RETURNING id,account_id,name,image_name,created,modified
    ";
    let thing: Thing = sqlx::query_as(query)
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
