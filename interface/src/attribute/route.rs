use std::sync::Arc;

use axum::{
  routing::{get, post},
  Router,
};
use infra::state::AppState;

use super::handler::{create_attribute, list_paginated_attributes};
pub struct AttributeRouter {}

impl AttributeRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new()
      .route("/attributes.create", post(create_attribute))
      .route("/attributes.list", get(list_paginated_attributes))
  }
}
