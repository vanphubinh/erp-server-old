use std::sync::Arc;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  Json,
};
use axum_macros::debug_handler;
use domain::product::product::QueryProductResult;
use infra::{
  response::{CreateResponse, PaginatedResponse},
  state::AppState,
};
use service::product::{
  list_paginated_products_usecase::{
    ListPaginatedProductsError, ListPaginatedProductsParams, ListPaginatedProductsUsecase,
  },
  CreateProductError, CreateProductPayload, CreateProductUsecase,
};

#[debug_handler]
pub async fn list_paginated_products(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedProductsParams>,
) -> Result<PaginatedResponse<QueryProductResult>, ListPaginatedProductsError> {
  let usecase = ListPaginatedProductsUsecase {
    page: Some(query.page.unwrap_or(1)),
    per_page: Some(query.per_page.unwrap_or(30)),
  };

  let (products, meta) = usecase.invoke(state.read_db.clone()).await?;

  Ok(PaginatedResponse::<QueryProductResult> {
    ok: true,
    data: products,
    meta,
  })
}

#[debug_handler]
pub async fn create_product(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateProductPayload>,
) -> Result<(StatusCode, CreateResponse), CreateProductError> {
  let usecase = CreateProductUsecase {
    name: payload.name,
    product_type: payload.product_type,
    product_subtype: payload.product_subtype,
    is_track_inventory: payload.is_track_inventory,
    price: payload.price,
    cost: payload.cost,
    uom_id: payload.uom_id,
    category_id: payload.category_id,
    create_corresponding_moulds: payload.create_corresponding_moulds,
    is_multiple_variants: payload.is_multiple_variants,
  };

  let product = usecase.invoke(state.write_db.clone()).await?;

  Ok((
    StatusCode::CREATED,
    CreateResponse {
      id: product.id,
      ok: true,
    },
  ))
}
