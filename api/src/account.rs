use axum::http::StatusCode;
use axum::{extract::State, http::HeaderMap};
use bitflags::bitflags;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::*;

use crate::app::{ApiError, AppState};
use crate::util::parse_jwt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IDPClaims {
    pub email: String,
}

/// An account for the site, stored in the DB
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Account {
    pub id: i32,
    pub role: Role,
    pub email: String,
}

/// Preconfigured settings for an account.
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct AccountSettings {
    pub role: Role,
    pub email: String,
}

/// Claims for an [`Account`], as part of a JWT
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct AccountClaims {
    pub id: i32,
    pub role: Role,
    pub email: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Type, Copy, Clone, Eq, PartialEq, Default, Debug)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Root,
    Admin,
    #[default]
    Basic,
}

impl Role {
    pub fn permissions(self) -> Permissions {
        match self {
            Role::Root => Role::Admin.permissions() | Permissions::CAN_ALTER_ACCOUNTS,
            Role::Admin => {
                Role::Basic.permissions()
                    | Permissions::CAN_ALTER_THINGS
                    | Permissions::CAN_ALTER_ALTER_CATEGORIES
                    | Permissions::CAN_ALTER_RANKS
            }
            Role::Basic => Permissions::CAN_VOTE,
        }
    }
}

bitflags! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Permissions: u8 {
        const CAN_VOTE                      = 0b00000001;
        const CAN_ALTER_THINGS              = 0b00000010;
        const CAN_ALTER_ALTER_CATEGORIES    = 0b00000100;
        const CAN_ALTER_RANKS               = 0b00001000;
        const CAN_ALTER_ACCOUNTS            = 0b00010000;
    }
}

/// Exchanges an IDP token for an API token
pub async fn exchange_idp_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(StatusCode, String), ApiError> {
    // Validates IDP token and validates claims
    let client_id = &state.oidc_config.google.client_id;
    let idp_jwt = parse_jwt(&headers)?;
    let idp_claims: IDPClaims = state.jwks_client.decode(idp_jwt, &[client_id]).await?;
    // Upsert account using info from IDP token
    let account = upsert_account(&idp_claims.email, Role::Basic, &state.pool).await?;
    // Return JWT
    let account_jwt = create_login_token(
        account,
        &state.auth_config.jwt_secret,
        Utc::now().timestamp() + state.auth_config.jwt_exp_secs,
    )
    .await?;
    Ok((StatusCode::CREATED, account_jwt))
}

pub async fn create_login_token(
    account: Account,
    secret: &str,
    jwt_exp_seconds: i64,
) -> Result<String, ApiError> {
    let encoding_key = &EncodingKey::from_base64_secret(secret)?;
    let account_claims = AccountClaims {
        id: account.id,
        role: account.role,
        email: account.email,
        exp: Utc::now().timestamp() + jwt_exp_seconds,
    };
    let account_jwt = encode(&Header::default(), &account_claims, &encoding_key)?;
    Ok(account_jwt)
}

pub async fn upsert_accounts(account_settings: &[AccountSettings], pool: &PgPool) {
    log::info!("Applying account settings");
    for settings in account_settings {
        upsert_account(&settings.email, settings.role, pool)
            .await
            .unwrap();
    }
}

/// Either inserts a new account, or updates an existing account.
/// Returns account.
pub async fn upsert_account(email: &str, role: Role, pool: &PgPool) -> Result<Account, ApiError> {
    let query = "SELECT id FROM account WHERE email=$1";
    let account_id: Option<i32> = sqlx::query_scalar(query)
        .bind(email)
        .fetch_optional(pool)
        .await?;
    let account_id = match account_id {
        Some(account_id) => update_account_role(account_id, role, pool).await?,
        None => create_account(email, role, pool).await?,
    };
    Ok(account_id)
}

pub async fn get_account_by_email(email: &str, pool: &PgPool) -> Result<Option<Account>, ApiError> {
    let query = "SELECT id,role,email FROM account WHERE email=$1";
    let account: Option<Account> = sqlx::query_as(query)
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(account)
}

async fn create_account(email: &str, role: Role, pool: &PgPool) -> Result<Account, ApiError> {
    let query = "INSERT INTO account (role,email) VALUES ($1,$2) RETURNING id,role,email";
    let account: Account = sqlx::query_as(query)
        .bind(role)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(account)
}

async fn update_account_role(id: i32, role: Role, pool: &PgPool) -> Result<Account, ApiError> {
    let query = "UPDATE account SET role=$1 WHERE id=$2 RETURNING id,role,email";
    let account: Account = sqlx::query_as(query)
        .bind(role)
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(account)
}
