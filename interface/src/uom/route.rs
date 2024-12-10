use std::sync::Arc;

use axum::Router;
use infra::state::AppState;
pub struct UomRouter {}

impl UomRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new()
  }
}
