use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Attribute::Table)
          .if_not_exists()
          .col(uuid(Attribute::Id).primary_key())
          .col(text(Attribute::Name))
          .col(timestamp_with_time_zone(Attribute::CreatedAt))
          .col(timestamp_with_time_zone(Attribute::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Attribute::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Attribute {
  Table,
  Id,
  Name,
  CreatedAt,
  UpdatedAt,
}
