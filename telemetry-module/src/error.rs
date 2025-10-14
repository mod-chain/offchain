use axum::{
  http::StatusCode,
  response::{IntoResponse, Response}
};
use serde_json::json;

pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "error": self.0.to_string() }).to_string()
        ).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}