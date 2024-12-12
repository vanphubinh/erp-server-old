use std::sync::Arc;

use axum::{routing::get, Router};
use infra::state::AppState;

use super::handler::list_paginated_categories;
pub struct CategoryRouter {}

impl CategoryRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new().route("/categories.list", get(list_paginated_categories))
  }
}
