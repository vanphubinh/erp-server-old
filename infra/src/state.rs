use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
  pub write_db: DatabaseConnection,
  pub read_db: DatabaseConnection,
}

impl AppState {
  pub fn new(write_db: DatabaseConnection, read_db: DatabaseConnection) -> Self {
    Self { write_db, read_db }
  }
}
