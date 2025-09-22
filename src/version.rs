use serde::{Serialize, Deserialize};
use axum::{
  RequestPartsExt,
  extract::{FromRequestParts, Path},
  http::{StatusCode, request::Parts},
  response::{IntoResponse, Response},
};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Version {
    V1,
}

impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params.get("version").ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                serde_json::json!({"error": "Version Param Missing"}).to_string(),
            )
                .into_response()
        })?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            _ => Err((
                StatusCode::NOT_FOUND,
                serde_json::json!({"error": "Unknown Version"}).to_string(),
            )
                .into_response()),
        }
    }
}