use std::sync::Arc;

use axum::{
  routing::{get, post},
  Router,
};
use infra::state::AppState;

use super::handler::{create_uom, find_uom, list_paginated_uoms, update_uom};
pub struct UomRouter {}

impl UomRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new()
      .route("/uoms.list", get(list_paginated_uoms))
      .route("/uoms.create", post(create_uom))
      .route("/uoms.find/:id", get(find_uom))
      .route("/uoms.update", post(update_uom))
  }
}
