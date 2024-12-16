use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(AttributeOption::Table)
          .if_not_exists()
          .col(uuid(AttributeOption::Id).primary_key())
          .col(text(AttributeOption::Value).default(""))
          .col(uuid(AttributeOption::AttributeId))
          .foreign_key(
            ForeignKey::create()
              .name("fk-attribute_option-attribute_id")
              .from(AttributeOption::Table, AttributeOption::AttributeId)
              .to(Attribute::Table, Attribute::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(AttributeOption::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum AttributeOption {
  Table,
  Id,
  Value,
  AttributeId,
}

#[derive(DeriveIden)]
enum Attribute {
  Table,
  Id,
}
