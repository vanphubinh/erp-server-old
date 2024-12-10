use discern::command::CommandBus;
use discern::query::QueryBus;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
  pub write_db: DatabaseConnection,
  pub read_db: DatabaseConnection,
  pub query_bus: QueryBus,
  pub command_bus: CommandBus,
}

impl AppState {
  pub fn new(
    write_db: DatabaseConnection,
    read_db: DatabaseConnection,
    query_bus: QueryBus,
    command_bus: CommandBus,
  ) -> Self {
    Self {
      write_db,
      read_db,
      query_bus,
      command_bus,
    }
  }
}
