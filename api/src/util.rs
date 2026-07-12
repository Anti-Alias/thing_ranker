use axum::http::HeaderMap;
use axum::http::header;

use crate::app::ApiError;

// Parses JWT portion of bearer token.
pub fn parse_bearer_token(headers: &HeaderMap) -> Result<&str, ApiError> {
    // Gets auth header parts
    let Some(auth_header) = headers.get(header::AUTHORIZATION) else {
        return Err(ApiError::AuthHeaderMissing);
    };
    let auth_header = auth_header.to_str()?;
    let mut auth_header_parts = auth_header.split_whitespace();
    // Parses bearer and jwt portion
    let Some(bearer) = auth_header_parts.next() else {
        return Err(ApiError::AuthHeaderMalformed);
    };
    if !bearer.eq_ignore_ascii_case("bearer") {
        return Err(ApiError::AuthHeaderMissingBearer);
    }
    let Some(jwt) = auth_header_parts.next() else {
        return Err(ApiError::AuthHeaderMissingJWT);
    };
    Ok(jwt)
}
