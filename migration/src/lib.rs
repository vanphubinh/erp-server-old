pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20241210_145837_create_uom_table;
mod m20241211_125011_create_category_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20241210_145837_create_uom_table::Migration),
      Box::new(m20241211_125011_create_category_table::Migration),
    ]
  }
}
