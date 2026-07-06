use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::decode_header;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Claims {
    iss: String,
    email: String,
}

/// Middleware that validates a JWT from an identify provider.
/// Used for signing users in.
pub async fn validate_claims(request: Request, next: Next) -> Result<Response, StatusCode> {
    let jwt = parse_auth_header(&request)?;
    let jwt_header = match decode_header(jwt) {
        Ok(header) => header,
        Err(err) => {
            log::error!("Failed to decode JWT header: {err}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let response = next.run(request).await;
    Ok(response)
}

/// Parses the auth header of a request, and returns a JWT
fn parse_auth_header(auth_header: &Request) -> Result<&str, StatusCode> {
    let auth_header = match auth_header.headers().get("Authorization") {
        Some(auth_header) => auth_header,
        None => {
            log::error!("Missing Authorization header");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let auth_header = match auth_header.to_str() {
        Ok(header) => header,
        Err(err) => {
            log::error!("Failed to parse auth header: {err}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let mut auth_header_parts = auth_header.split_whitespace();
    let bearer = match auth_header_parts.next() {
        Some(bearer) => bearer,
        None => {
            log::error!("Missing bearer in auth header");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    if !bearer.eq_ignore_ascii_case("bearer") {
        log::error!("Missing 'bearer' portion of authorization header");
        return Err(StatusCode::BAD_REQUEST);
    }
    let jwt = match auth_header_parts.next() {
        Some(jwt) => jwt,
        None => {
            log::error!("Missing JWT portion of authorization header");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    Ok(jwt)
}
