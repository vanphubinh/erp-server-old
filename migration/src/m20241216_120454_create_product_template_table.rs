use sea_orm::EnumIter;
use sea_orm_migration::prelude::{sea_query::extension::postgres::Type, *};
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_type(
        Type::create()
          .as_enum(ProductType::Enum)
          .values([ProductType::Goods, ProductType::Service])
          .to_owned(),
      )
      .await?;

    manager
      .create_type(
        Type::create()
          .as_enum(ProductSubtype::Enum)
          .values([
            ProductSubtype::Normal,
            ProductSubtype::PackagingWithPrint,
            ProductSubtype::Mould,
          ])
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(ProductTemplate::Table)
          .if_not_exists()
          .col(uuid(ProductTemplate::Id).primary_key())
          .col(text(ProductTemplate::Name).default(""))
          .col(
            ColumnDef::new(ProductTemplate::ProductType)
              .custom(ProductType::Enum)
              .not_null()
              .default(ProductType::Goods.to_string()),
          )
          .col(
            ColumnDef::new(ProductTemplate::ProductSubtype)
              .custom(ProductSubtype::Enum)
              .not_null()
              .default(ProductSubtype::Normal.to_string()),
          )
          .col(text(ProductTemplate::Description).default(""))
          .col(boolean(ProductTemplate::IsTrackInventory).default(true))
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
      .await?;
    manager
      .drop_type(Type::drop().name(ProductType::Enum).to_owned())
      .await?;
    manager
      .drop_type(Type::drop().name(ProductSubtype::Enum).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum ProductTemplate {
  Table,
  Id,
  Name,
  ProductType,
  ProductSubtype,
  IsTrackInventory,
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

#[derive(DeriveIden, EnumIter)]
enum ProductType {
  #[sea_orm(iden = "product_type")]
  Enum,
  #[sea_orm(iden = "goods")]
  Goods,
  #[sea_orm(iden = "service")]
  Service,
}

#[derive(DeriveIden, EnumIter)]
enum ProductSubtype {
  #[sea_orm(iden = "product_subtype")]
  Enum,
  #[sea_orm(iden = "normal")]
  Normal,
  #[sea_orm(iden = "packaging_with_print")]
  PackagingWithPrint,
  #[sea_orm(iden = "mould")]
  Mould,
}
