//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, ActiveModelTrait, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "attribute")]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::attribute_option::Entity")]
  AttributeOption,
}

impl Related<super::attribute_option::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AttributeOption.def()
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
    if insert {
      let now = Utc::now().into();
      this.created_at = Set(now);
      this.updated_at = Set(now);
    } else {
      this.updated_at = Set(Utc::now().into());
    }
    Ok(this)
  }
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct QueryResult {
  pub id: Uuid,
  pub name: String,
  pub attribute_option_id: Option<Uuid>,
  pub attribute_option_value: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AttributeDTO {
  pub id: Uuid,
  pub name: String,
  pub attribute_options: Vec<super::attribute_option::PartialModel>,
}
