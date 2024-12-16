use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(ProductTemplate::Table)
          .if_not_exists()
          .col(uuid(ProductTemplate::Id).primary_key())
          .col(text(ProductTemplate::Name).default(""))
          .col(text(ProductTemplate::Description).default(""))
          .col(uuid(ProductTemplate::UomId))
          .col(uuid_null(ProductTemplate::CategoryId))
          .col(
            timestamp_with_time_zone(ProductTemplate::CreatedAt).default(Expr::current_timestamp()),
          )
          .col(timestamp_with_time_zone_null(ProductTemplate::UpdatedAt))
          .col(timestamp_with_time_zone_null(ProductTemplate::ArchivedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-product_template-uom_id")
              .from(ProductTemplate::Table, ProductTemplate::UomId)
              .to(Uom::Table, Uom::Id),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-product_template-category_id")
              .from(ProductTemplate::Table, ProductTemplate::CategoryId)
              .to(Category::Table, Category::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(ProductTemplate::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum ProductTemplate {
  Table,
  Id,
  Name,
  Description,
  UomId,
  CategoryId,
  CreatedAt,
  UpdatedAt,
  ArchivedAt,
}

#[derive(DeriveIden)]
enum Uom {
  Table,
  Id,
}

#[derive(DeriveIden)]
enum Category {
  Table,
  Id,
}
