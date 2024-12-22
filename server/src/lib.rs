use axum::{
  http::{header::CONTENT_TYPE, Method},
  Router,
};
use infra::state::AppState;
use interface::{
  attribute::route::AttributeRouter, category::route::CategoryRouter,
  product::route::ProductRouter, uom::route::UomRouter,
};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{
  cors::{Any, CorsLayer},
  trace::{self, TraceLayer},
};
use tracing::Level;

#[tokio::main]
pub async fn start() {
  dotenvy::dotenv().ok();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

  tracing::info!("Connecting to databases...");
  let write_db = match get_db_connection("DATABASE_URL").await {
    Ok(db) => {
      tracing::info!("Connected to write database!");
      db
    }
    Err(_) => {
      tracing::error!("Failed to connect to write database!");
      return;
    }
  };
  let read_db = match get_db_connection("DATABASE_URL_READ").await {
    Ok(db) => {
      tracing::info!("Connected to read database!");
      db
    }
    Err(_) => {
      tracing::error!("Failed to connect to read database!");
      return;
    }
  };
  let app_state = Arc::new(AppState { write_db, read_db });
  let port: u16 = std::env::var("PORT")
    .unwrap_or("3000".into())
    .parse()
    .expect("failed to convert to number");

  let ipv6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));

  let ipv6_listener = TcpListener::bind(&ipv6).await.unwrap();

  let cors = CorsLayer::new()
    .allow_methods([
      Method::GET,
      Method::POST,
      Method::OPTIONS,
      Method::PUT,
      Method::DELETE,
    ])
    .allow_origin(Any)
    .allow_headers([CONTENT_TYPE]);

  let router = Router::new()
    .merge(UomRouter::new())
    .merge(CategoryRouter::new())
    .merge(AttributeRouter::new())
    .merge(ProductRouter::new())
    .layer(cors)
    .layer(
      TraceLayer::new_for_http().make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)),
    )
    .with_state(app_state.clone());

  tracing::debug!("Listening on http://{}", ipv6);
  axum::serve(ipv6_listener, router).await.unwrap();
}

async fn get_db_connection(env_var: &str) -> Result<DatabaseConnection, DbErr> {
  let db_url = std::env::var(env_var).unwrap();
  let db = Database::connect(&db_url).await?;
  Ok(db)
}
