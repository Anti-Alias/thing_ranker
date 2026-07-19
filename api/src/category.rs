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
    util::{Order, decode_cursor, escape_like_query},
};

const CATEGORY_PAGE_SIZE: i32 = 15;

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

#[derive(Deserialize, Debug)]
pub struct CategoryQueryParams {
    order: Option<Order>,
    cursor: Option<String>,
    name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryPage {
    pub items: Vec<Category>,
    pub cursor: Option<String>,
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

pub async fn get_category_page(
    Query(params): Query<CategoryQueryParams>,
    State(state): State<AppState>,
) -> ApiResponse<CategoryPage> {
    // Gets a page of categories + 1 extra entry
    let mut categories = {
        let cursor = decode_cursor(params.cursor)?;
        let order = params.order.unwrap_or_default();
        // Base query
        let base_query = "SELECT id,account_id,name,image,created,modified FROM category WHERE 1=1";
        let mut builder = QueryBuilder::<Postgres>::new(base_query);
        // Filter by name similarity
        if let Some(name) = params.name {
            let name = escape_like_query(&name);
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
        // Limit
        builder.push(" LIMIT ").push_bind(CATEGORY_PAGE_SIZE + 1);
        builder
            .build_query_as::<Category>()
            .fetch_all(&state.pool)
            .await?
    };
    // Returns category page, with cursor if there are more rows
    let has_more_rows = categories.len() as i32 == CATEGORY_PAGE_SIZE + 1;
    let category_page = if has_more_rows {
        let last_category = categories.pop().unwrap();
        let cursor = Some(BASE64_STANDARD.encode(&last_category.name));
        CategoryPage {
            items: categories,
            cursor,
        }
    } else {
        CategoryPage {
            items: categories,
            cursor: None,
        }
    };
    return Ok((StatusCode::OK, Json(category_page)));
}

pub async fn create_category(
    State(state): State<AppState>,
    Extension(claims): Extension<AccountClaims>,
    TypedMultipart(request): TypedMultipart<CreateCategoryRequest>,
) -> ApiResponse<Category> {
    // Insert category in DB
    if category_exists(&request.name, &state.pool).await? {
        return Err(ApiError::CategoryAlreadyExists);
    }
    let image_name = uuid::Uuid::new_v4().to_string();
    let image_name = format!("{image_name}.webp");
    let query = "
        INSERT INTO category (account_id, name, image)
        VALUES ($1, $2, $3)
        RETURNING id,account_id,name,image,created,modified
    ";
    let category: Category = sqlx::query_as(query)
        .bind(claims.id)
        .bind(&request.name)
        .bind(&image_name)
        .fetch_one(&state.pool)
        .await?;
    // Write image bytes to asset store
    let image_bytes = process_image(&request.file.contents)?;
    state
        .asset_store
        .write("images", &image_name, &image_bytes)
        .await?;
    Ok((StatusCode::OK, Json(category)))
}
async fn category_exists(name: &str, pool: &PgPool) -> Result<bool, ApiError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM category WHERE name=$1")
        .bind(name)
        .fetch_one(pool)
        .await?;
    Ok(count >= 1)
}
