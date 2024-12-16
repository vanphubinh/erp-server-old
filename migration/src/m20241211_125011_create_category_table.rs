use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Category::Table)
          .if_not_exists()
          .col(uuid(Category::Id).primary_key())
          .col(text(Category::Name).default(""))
          .col(uuid_null(Category::ParentCategoryId))
          .col(timestamp_with_time_zone(Category::CreatedAt).default(Expr::current_timestamp()))
          .col(timestamp_with_time_zone_null(Category::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-category-parent_category_id")
              .from(Category::Table, Category::ParentCategoryId)
              .to(Category::Table, Category::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Category::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Category {
  Table,
  Id,
  Name,
  ParentCategoryId,
  CreatedAt,
  UpdatedAt,
}
