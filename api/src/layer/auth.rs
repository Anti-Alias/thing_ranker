use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    account::AccountClaims,
    app::{ApiError, AppState},
    util::parse_bearer_token,
};

pub async fn authenticate(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Extracts / validates claim from AUTHORIZATION header
    let jwt = parse_bearer_token(req.headers())?;
    let decoding_key = DecodingKey::from_base64_secret(&state.auth_config.jwt_secret)?;
    let account_claims: AccountClaims = decode(jwt, &decoding_key, &Validation::default())?.claims;
    req.extensions_mut().insert(account_claims);
    // Continues to route
    let resp = next.run(req).await;
    Ok(resp)
}
