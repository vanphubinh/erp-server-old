use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Uom::Table)
          .if_not_exists()
          .col(uuid(Uom::Id).primary_key())
          .col(text(Uom::Name).default(""))
          .col(timestamp_with_time_zone(Uom::CreatedAt).default(Expr::current_timestamp()))
          .col(timestamp_with_time_zone_null(Uom::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Uom::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Uom {
  Table,
  Id,
  Name,
  CreatedAt,
  UpdatedAt,
}
