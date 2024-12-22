use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

use super::{attribute, attribute_option};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Copy)]
#[sea_orm(table_name = "product")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub product_template_id: Uuid,
  pub price: Decimal,
  pub cost: Decimal,
  pub is_product_variant: bool,
  pub created_at: ChronoDateTimeWithTimeZone,
  #[sea_orm(nullable)]
  pub updated_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::product_template::Entity",
    from = "Column::ProductTemplateId",
    to = "super::product_template::Column::Id"
  )]
  ProductTemplate,
}

impl Related<super::product_template::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ProductTemplate.def()
  }
}

impl Related<super::attribute_option::Entity> for Entity {
  fn to() -> RelationDef {
    super::product_combination::Relation::AttributeOption.def()
  }

  fn via() -> Option<RelationDef> {
    Some(super::product_combination::Relation::Product.def().rev())
  }
}

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

#[derive(Debug, FromQueryResult)]
pub struct QueryProductResult {
  pub id: Uuid,
  pub name: String,
  pub is_product_variant: bool,
  pub product_template_id: Option<Uuid>,
  pub attribute_id: Option<Uuid>,
  pub attribute_name: Option<String>,
  pub attribute_option_id: Option<Uuid>,
  pub attribute_option_value: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductDTO {
  pub id: Uuid,
  pub name: String,
  pub is_product_variant: bool,
  pub combinations: Vec<AttributeWithOptionDTO>,
}

#[derive(Debug, Serialize)]
pub struct AttributeWithOptionDTO {
  pub attribute: attribute::PartialModel,
  pub option: attribute_option::PartialModel,
}
