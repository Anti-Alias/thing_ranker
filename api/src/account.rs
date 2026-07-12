use axum::http::StatusCode;
use axum::{extract::State, http::HeaderMap};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::{FromRow, Type};

use crate::app::{ApiError, AppState};
use crate::util::parse_bearer_token;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IDPClaims {
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Account {
    pub id: i32,
    pub role: Role,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct AccountClaims {
    pub id: i32,
    pub role: Role,
    pub email: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Type, Copy, Clone, Eq, PartialEq, Default, Debug)]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
    Root,
    Admin,
    #[default]
    Basic,
}

pub async fn create_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(StatusCode, String), ApiError> {
    // Validates IDP token and validates claims
    let client_id = &state.oidc_config.google.client_id;
    let idp_jwt = parse_bearer_token(&headers)?;
    let idp_claims: IDPClaims = state.jwks_client.decode(idp_jwt, &[client_id]).await?;
    // Get or create account using info form IDP token
    let encoding_key = &EncodingKey::from_base64_secret(&state.auth_config.jwt_secret)?;
    let account = get_or_create_account(&idp_claims.email, &state.pool).await?;
    // Return JWT
    let account_claims = AccountClaims {
        id: account.id,
        role: account.role,
        email: account.email,
        exp: Utc::now().timestamp() + state.auth_config.jwt_exp_secs,
    };
    let account_jwt = encode(&Header::default(), &account_claims, &encoding_key)?;
    Ok((StatusCode::CREATED, account_jwt))
}

async fn get_or_create_account(email: &str, pool: &PgPool) -> Result<Account, ApiError> {
    const QUERY: &str = "SELECT id,role,email FROM account WHERE email=$1";
    let account: Option<Account> = sqlx::query_as(QUERY)
        .bind(email)
        .fetch_optional(pool)
        .await?;
    let account = match account {
        Some(account) => account,
        None => create_account(email, pool).await?,
    };
    Ok(account)
}

async fn create_account(email: &str, pool: &PgPool) -> Result<Account, ApiError> {
    const QUERY: &str =
        "INSERT INTO account (role,email) VALUES ('basic',$1) RETURNING id,role,email";
    let account: Account = sqlx::query_as(QUERY).bind(email).fetch_one(pool).await?;
    Ok(account)
}
