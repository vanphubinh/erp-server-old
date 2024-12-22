use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(ProductCombination::Table)
          .if_not_exists()
          .col(uuid(ProductCombination::ProductId))
          .col(uuid(ProductCombination::AttributeOptionId))
          .primary_key(
            Index::create()
              .name("pk-product_combination")
              .col(ProductCombination::ProductId)
              .col(ProductCombination::AttributeOptionId),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-product_combination-product_id")
              .from(ProductCombination::Table, ProductCombination::ProductId)
              .to(Product::Table, Product::Id),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-product_combination-attribute_option_id")
              .from(
                ProductCombination::Table,
                ProductCombination::AttributeOptionId,
              )
              .to(AttributeOption::Table, AttributeOption::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(ProductCombination::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum ProductCombination {
  Table,
  ProductId,
  AttributeOptionId,
}

#[derive(DeriveIden)]
enum Product {
  Table,
  Id,
}

#[derive(DeriveIden)]
enum AttributeOption {
  Table,
  Id,
}
