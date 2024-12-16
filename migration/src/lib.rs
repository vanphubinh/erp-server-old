pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20241210_145837_create_uom_table;
mod m20241211_125011_create_category_table;
mod m20241212_025514_create_attribute_table;
mod m20241212_025826_create_attribute_option_table;
mod m20241216_120454_create_product_template_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20241210_145837_create_uom_table::Migration),
            Box::new(m20241211_125011_create_category_table::Migration),
            Box::new(m20241212_025514_create_attribute_table::Migration),
            Box::new(m20241212_025826_create_attribute_option_table::Migration),
            Box::new(m20241216_120454_create_product_template_table::Migration),
        ]
  }
}
