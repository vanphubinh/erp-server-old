use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use infra::{response::CreateResponse, state::AppState, uuid::Uuid};
use service::catalog::{CreateAttributeError, CreateAttributePayload};
use std::sync::Arc;

#[debug_handler]
pub async fn create_attribute(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateAttributePayload>,
) -> Result<(StatusCode, CreateResponse), CreateAttributeError> {
  let usecase = CreateAttributePayload {
    name: payload.name,
    attribute_options: payload.attribute_options,
  };

  let created_attribute = usecase.invoke(state.write_db.clone()).await?;

  Ok((
    StatusCode::CREATED,
    CreateResponse {
      id: created_attribute.id,
      ok: true,
    },
  ))
}
