use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "product_template")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  #[sea_orm(column_type = "Text")]
  pub description: String,
  pub uom_id: Uuid,
  #[sea_orm(nullable)]
  pub category_id: Option<Uuid>,
  pub product_type: ProductType,
  pub product_subtype: ProductSubtype,
  pub is_track_inventory: bool,
  pub created_at: ChronoDateTimeWithTimeZone,
  #[sea_orm(nullable)]
  pub updated_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new()),
      ..ActiveModelTrait::default()
    }
  }

  async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let _ = db;
    let mut this = self;
    if !insert {
      this.updated_at = Set(Some(Utc::now().into()));
    }
    Ok(this)
  }
}
#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult)]
#[sea_orm(entity = "Entity")]
#[serde(rename_all = "camelCase")]
pub struct PartialModel {
  pub id: Uuid,
  pub name: String,
}

#[derive(Debug, EnumIter, DeriveActiveEnum, Deserialize, Clone, PartialEq, Eq, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "product_type")]
pub enum ProductType {
  #[sea_orm(string_value = "goods")]
  #[serde(rename(deserialize = "goods"))]
  Goods,
  #[sea_orm(string_value = "service")]
  #[serde(rename(deserialize = "service"))]
  Service,
}

#[derive(Debug, EnumIter, DeriveActiveEnum, Deserialize, Clone, PartialEq, Eq, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "product_subtype")]
pub enum ProductSubtype {
  #[sea_orm(string_value = "normal")]
  #[serde(rename(deserialize = "normal"))]
  Normal,
  #[sea_orm(string_value = "packaging_with_print")]
  #[serde(rename(deserialize = "packaging_with_print"))]
  PackagingWithPrint,
  #[sea_orm(string_value = "mould")]
  #[serde(rename(deserialize = "mould"))]
  Mould,
}
