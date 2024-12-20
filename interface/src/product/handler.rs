use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use infra::{response::CreateResponse, state::AppState};
use service::product::{CreateProductError, CreateProductPayload, CreateProductUsecase};

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
