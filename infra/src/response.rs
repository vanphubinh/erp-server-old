use axum::{response::IntoResponse, response::Response, Json};
use serde::Serialize;

use crate::uuid::Uuid;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub ok: bool,
  pub code: String,
  pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
  pub page: u64,
  pub total_pages: u64,
  pub per_page: u64,
  pub total: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
  pub ok: bool,
  pub data: Vec<T>,
  pub meta: PaginationMeta,
}

impl<T> IntoResponse for PaginatedResponse<T>
where
  T: Serialize,
{
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

#[derive(Serialize)]
pub struct CreateResponse {
  pub ok: bool,
  pub id: Uuid,
}

impl IntoResponse for CreateResponse {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

#[derive(Serialize)]
pub struct FindOneResponse<T> {
  pub ok: bool,
  pub data: T,
}

impl<T> IntoResponse for FindOneResponse<T>
where
  T: Serialize,
{
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

#[derive(Serialize)]
pub struct OkResponse {
  pub ok: bool,
}

impl IntoResponse for OkResponse {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse<T> {
  pub ok: bool,
  pub data: T,
}

impl<T> IntoResponse for QueryResponse<T>
where
  T: Serialize,
{
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}
