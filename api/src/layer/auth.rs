use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    account::{AccountClaims, Role},
    app::{ApiError, AppState},
    util::parse_jwt,
};

pub async fn authenticate(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Parses JWT from bearer token within authorization header
    let jwt = parse_jwt(req.headers())?;
    // Validates JWT
    let decoding_key = DecodingKey::from_base64_secret(&state.auth_config.jwt_secret)?;
    let account_claims: AccountClaims = decode(jwt, &decoding_key, &Validation::default())?.claims;
    req.extensions_mut().insert(account_claims);
    // Continues to route
    let resp = next.run(req).await;
    Ok(resp)
}

pub async fn authorize_admin(req: Request, next: Next) -> Result<Response, ApiError> {
    // Gets claims from extension
    let claims = req
        .extensions()
        .get::<AccountClaims>()
        .expect("auhorize_admin() middleware must be wrapped in authenticate() middleware");
    // Ensures user is an admin
    if claims.role != Role::Admin && claims.role != Role::Root {
        return Err(ApiError::Unauthorized);
    }
    // Continues to route
    let resp = next.run(req).await;
    Ok(resp)
}
