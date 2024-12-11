use axum::{
  extract::{Json, Query, State},
  http::StatusCode,
};
use axum_macros::debug_handler;
use domain::measurement::uom::Model as Uom;
use infra::{response::PaginatedResponse, state::AppState};
use service::measurement::{
  CreateUomError, CreateUomParams, CreateUomUsecase, ListPaginatedUomsError,
  ListPaginatedUomsParams, ListPaginatedUomsUsecase,
};
use std::sync::Arc;

#[debug_handler]
pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedUomsParams>,
) -> Result<PaginatedResponse<Uom>, ListPaginatedUomsError> {
  let usecase = ListPaginatedUomsUsecase {
    page: Some(query.page.unwrap_or(1)),
    per_page: Some(query.per_page.unwrap_or(30)),
  };

  let (uoms, meta) = usecase.invoke(state.read_db.clone()).await?;

  Ok(PaginatedResponse::<Uom> {
    ok: true,
    data: uoms,
    meta,
  })
}

#[debug_handler]
pub async fn create_uom(
  State(state): State<Arc<AppState>>,
  Json(body): Json<CreateUomParams>,
) -> Result<(StatusCode, Json<Uom>), CreateUomError> {
  let usecase = CreateUomUsecase { name: body.name };

  let uom = usecase.invoke(state.read_db.clone()).await?;

  Ok((StatusCode::CREATED, Json(uom)))
}
