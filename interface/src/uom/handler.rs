use axum::extract::{Query, State};
use axum_macros::debug_handler;
use domain::measurement::uom::Model as UomModel;
use infra::{response::PaginatedResponse, state::AppState};
use service::measurement::{
  ListPaginatedUomsError, ListPaginatedUomsParams, ListPaginatedUomsUseCase,
};
use std::sync::Arc;

#[debug_handler]
pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedUomsParams>,
) -> Result<PaginatedResponse<UomModel>, ListPaginatedUomsError> {
  let usecase = ListPaginatedUomsUseCase {
    page: Some(query.page.unwrap_or(1)),
    per_page: Some(query.per_page.unwrap_or(30)),
  };

  let (uoms, meta) = usecase.invoke(state.read_db.clone()).await?;

  Ok(PaginatedResponse::<UomModel> {
    ok: true,
    data: uoms,
    meta,
  })
}
