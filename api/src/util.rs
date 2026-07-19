use axum::http::HeaderMap;
use axum::http::header;
use base64::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::app::ApiError;

pub const STATIC_FILE_BASE_PATH: &str = "assets";

/// Commonly used sorting order
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    #[default]
    Asc,
    Desc,
}

// Parses JWT portion of bearer token.
pub fn parse_jwt(headers: &HeaderMap) -> Result<&str, ApiError> {
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

/// Decodes an optional Base64 encoded cursor its original string value.
pub fn decode_cursor(cursor: Option<String>) -> Result<Option<String>, ApiError> {
    let name_bytes = match cursor {
        Some(cursor) => BASE64_STANDARD
            .decode(cursor)
            .map_err(|_| ApiError::Base64DecodingFailed)?,
        None => return Ok(None),
    };
    let name_string = String::from_utf8(name_bytes).map_err(|_| ApiError::Base64DecodingFailed)?;
    Ok(Some(name_string))
}

/// Escapes special characters in a like / ilike query
pub fn escape_like_query(value: &str) -> String {
    let mut result = String::new();
    for c in value.chars() {
        let is_special_char = c == '_' || c == '%' || c == '\\';
        if is_special_char {
            result.push('\\');
        }
        result.push(c);
    }
    result
}
