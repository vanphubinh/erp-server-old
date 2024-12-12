use axum::extract::{Query, State};
use axum_macros::debug_handler;
use domain::catalog::category::PartialModel as Category;
use infra::{response::PaginatedResponse, state::AppState};
use service::catalog::{
  ListPaginatedCategoriesError, ListPaginatedCategoriesParams, ListPaginatedCategoriesUsecase,
};
use std::sync::Arc;

#[debug_handler]
pub async fn list_paginated_categories(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedCategoriesParams>,
) -> Result<PaginatedResponse<Category>, ListPaginatedCategoriesError> {
  let usecase = ListPaginatedCategoriesUsecase {
    page: Some(query.page.unwrap_or(1)),
    per_page: Some(query.per_page.unwrap_or(30)),
  };

  let (categories, meta) = usecase.invoke(state.read_db.clone()).await?;

  Ok(PaginatedResponse::<Category> {
    ok: true,
    data: categories,
    meta,
  })
}
