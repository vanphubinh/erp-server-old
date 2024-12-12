use std::sync::Arc;

use axum::{
  routing::{get, post},
  Router,
};
use infra::state::AppState;

use super::handler::{
  create_attribute, find_attribute, list_paginated_attributes, update_attribute,
};
pub struct AttributeRouter {}

impl AttributeRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new()
      .route("/attributes.create", post(create_attribute))
      .route("/attributes.list", get(list_paginated_attributes))
      .route("/attributes.find/:id", get(find_attribute))
      .route("/attributes.update", post(update_attribute))
  }
}
