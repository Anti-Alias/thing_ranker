use axum::{
    Extension, Json,
    body::Bytes,
    extract::{Path, Query, State},
    http::StatusCode,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use base64::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::{PgConnection, PgTransaction, Postgres, QueryBuilder};

use crate::{
    account::AccountClaims,
    app::{ApiError, ApiResponse, AppState},
    image::process_image,
    util::{Order, decode_cursor},
};

const THING_PAGE_SIZE: i32 = 15;

#[skip_serializing_none]
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub image: Option<String>,
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
    name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ThingPage {
    pub things: Vec<Thing>,
    pub cursor: Option<String>,
}

pub async fn get_thing(Path(id): Path<i32>, State(state): State<AppState>) -> ApiResponse<Thing> {
    let query = "
        SELECT id,account_id,name,image,created,modified
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
    // Gets a page of things + 1 extra entry
    let mut things = {
        let cursor = decode_cursor(params.cursor)?;
        let order = params.order.unwrap_or_default();
        let base_query = "SELECT id,account_id,name,image,created,modified FROM thing WHERE 1=1";
        let mut builder = QueryBuilder::<Postgres>::new(base_query);
        // Filter by name similarity
        if let Some(name) = params.name {
            if name.chars().nth(2).is_none() {
                return Err(ApiError::QueryStringTooSmall);
            }
            builder
                .push(" AND name ILIKE '%' || ")
                .push_bind(name)
                .push("|| '%'");
        }
        // Order / page logic
        match (cursor, order) {
            (Some(cursor), Order::Asc) => {
                builder.push(" AND name >= ").push_bind(cursor);
                builder.push(" ORDER BY name ASC");
            }
            (Some(cursor), Order::Desc) => {
                builder.push(" AND name <= ").push_bind(cursor);
                builder.push(" ORDER BY name DESC");
            }
            (None, Order::Asc) => {
                builder.push(" ORDER BY name ASC");
            }
            (None, Order::Desc) => {
                builder.push(" ORDER BY name DESC");
            }
        };
        builder.push(" LIMIT ").push_bind(THING_PAGE_SIZE + 1);
        builder
            .build_query_as::<Thing>()
            .fetch_all(&state.pool)
            .await?
    };
    // Returns thing page, with cursor if there are more rows
    let has_more_rows = things.len() as i32 == THING_PAGE_SIZE + 1;
    let thing_page = if has_more_rows {
        let last_thing = things.pop().unwrap();
        let cursor = Some(BASE64_STANDARD.encode(&last_thing.name));
        ThingPage { things, cursor }
    } else {
        ThingPage {
            things,
            cursor: None,
        }
    };
    return Ok((StatusCode::OK, Json(thing_page)));
}

pub async fn create_thing(
    State(state): State<AppState>,
    Extension(claims): Extension<AccountClaims>,
    TypedMultipart(request): TypedMultipart<CreateThingRequest>,
) -> ApiResponse<Thing> {
    // Transaction start
    let mut tx: PgTransaction = state.pool.begin().await?;
    let conn = &mut *tx;
    // Processes image bytes
    let image_bytes = process_image(&request.file.contents)?;
    // Insert thing in DB
    if thing_exists(&request.name, conn).await? {
        return Err(ApiError::ThingAlreadyExists);
    }
    let image_name = uuid::Uuid::new_v4().to_string();
    let image_name = format!("{image_name}.webp");
    let query = "
        INSERT INTO thing (account_id, name, image)
        VALUES ($1, $2, $3)
        RETURNING id,account_id,name,image,created,modified
    ";
    let thing: Thing = sqlx::query_as(query)
        .bind(claims.id)
        .bind(&request.name)
        .bind(&image_name)
        .fetch_one(conn)
        .await?;
    // Write image bytes to asset store
    state
        .asset_store
        .write("images", &image_name, &image_bytes)
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
