use std::sync::Arc;

use axum::{routing::post, Router};
use infra::state::AppState;

use super::handler::create_attribute;
pub struct AttributeRouter {}

impl AttributeRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new().route("/attributes.create", post(create_attribute))
  }
}
