use std::num::NonZeroI32;

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
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    account::AccountClaims,
    app::{ApiError, ApiResponse, AppState},
    image::process_image,
    util::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE, Order, decode_cursor, to_like_value},
};

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
#[serde(rename_all = "camelCase")]
pub struct ThingQueryParams {
    order: Option<Order>,
    cursor: Option<String>,
    name: Option<String>,
    category_id: Option<i32>,
    page_size: Option<NonZeroI32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct ThingPage {
    pub items: Vec<Thing>,
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
    Query(params): Query<ThingQueryParams>,
    State(state): State<AppState>,
) -> ApiResponse<ThingPage> {
    let page_size = params
        .page_size
        .map(|s| s.get())
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .min(MAX_PAGE_SIZE);
    let mut builder = QueryBuilder::<Postgres>::default();
    // Base query
    {
        if params.category_id.is_some() {
            builder.push("SELECT t.id,t.account_id,t.name,t.image,t.created,t.modified FROM thing t INNER JOIN rank r ON t.id = r.thing_id WHERE 1=1");
        } else {
            builder.push("SELECT t.id,t.account_id,t.name,t.image,t.created,t.modified FROM thing t WHERE 1=1");
        }
    }

    // Filter by name
    if let Some(name) = params.name {
        let name = to_like_value(&name);
        if name.chars().nth(2).is_none() {
            return Err(ApiError::QueryStringTooSmall);
        }
        builder.push(" AND t.name ILIKE ").push_bind(name);
    }

    // Filter by category
    if let Some(category_id) = params.category_id {
        builder.push(" AND r.category_id = ").push_bind(category_id);
    }

    // Order by name
    {
        let order = params.order.unwrap_or_default();
        let cursor = decode_cursor(params.cursor)?;
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
    }

    // Limit by page size
    builder.push(" LIMIT ").push_bind(page_size + 1);

    // Gets a page of things + 1 extra entry
    let mut things: Vec<Thing> = builder
        .build_query_as::<Thing>()
        .fetch_all(&state.pool)
        .await?;

    // Returns thing page, with cursor if there are more rows
    let has_more_rows = things.len() as i32 == page_size + 1;
    let thing_page = if has_more_rows {
        let last_thing = things.pop().unwrap();
        let cursor = Some(BASE64_STANDARD.encode(&last_thing.name));
        ThingPage {
            items: things,
            cursor,
        }
    } else {
        ThingPage {
            items: things,
            cursor: None,
        }
    };
    Ok((StatusCode::OK, Json(thing_page)))
}

pub async fn create_thing(
    State(state): State<AppState>,
    Extension(claims): Extension<AccountClaims>,
    TypedMultipart(request): TypedMultipart<CreateThingRequest>,
) -> ApiResponse<Thing> {
    let thing =
        create_thing_inner(&state, claims.id, &request.name, &request.file.contents).await?;
    Ok((StatusCode::CREATED, Json(thing)))
}

pub async fn create_thing_inner(
    state: &AppState,
    account_id: i32,
    thing_name: &str,
    thing_bytes: &[u8],
) -> Result<Thing, ApiError> {
    // Insert thing in DB
    if thing_exists(thing_name, &state.pool).await? {
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
        .bind(account_id)
        .bind(thing_name)
        .bind(&image_name)
        .fetch_one(&state.pool)
        .await?;
    // Write image bytes to asset store
    let image_bytes = process_image(thing_bytes)?;
    state
        .asset_store
        .write("images", &image_name, &image_bytes)
        .await?;
    Ok(thing)
}

async fn thing_exists(name: &str, pool: &PgPool) -> Result<bool, ApiError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM thing WHERE name=$1")
        .bind(name)
        .fetch_one(pool)
        .await?;
    Ok(count >= 1)
}
