use std::sync::Arc;

use axum::{routing::post, Router};
use infra::state::AppState;

use super::handler::create_product;
pub struct ProductRouter {}

impl ProductRouter {
  pub fn new() -> Router<Arc<AppState>> {
    Router::new().route("/products.create", post(create_product))
  }
}
