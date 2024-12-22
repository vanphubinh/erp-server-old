use std::sync::Arc;

use axum::{
  routing::{get, post},
  Router,
};
use infra::state::AppState;

use super::handler::{create_product, list_paginated_products};
pub struct ProductRouter {}

impl ProductRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new()
      .route("/products.list", get(list_paginated_products))
      .route("/products.create", post(create_product))
  }
}
