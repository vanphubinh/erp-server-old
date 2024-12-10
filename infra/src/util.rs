use crate::response::ErrorResponse;
use axum::{response::IntoResponse, Json};

pub fn error(code: String, source: Option<String>) -> impl IntoResponse {
  Json(ErrorResponse {
    ok: false,
    code,
    source,
  })
  .into_response()
}
