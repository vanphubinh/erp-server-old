use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Product::Table)
          .if_not_exists()
          .col(uuid(Product::Id).primary_key())
          .col(uuid(Product::ProductTemplateId))
          .col(decimal_len(Product::Price, 15, 3).default(0.0))
          .col(decimal_len(Product::Cost, 15, 3).default(0.0))
          .col(timestamp_with_time_zone(Product::CreatedAt).default(Expr::current_timestamp()))
          .col(timestamp_with_time_zone_null(Product::UpdatedAt))
          .col(timestamp_with_time_zone_null(Product::ArchivedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-product-product_template_id")
              .from(Product::Table, Product::ProductTemplateId)
              .to(ProductTemplate::Table, ProductTemplate::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Product::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Product {
  Table,
  Id,
  ProductTemplateId,
  Price,
  Cost,
  CreatedAt,
  UpdatedAt,
  ArchivedAt,
}

#[derive(DeriveIden)]
enum ProductTemplate {
  Table,
  Id,
}
