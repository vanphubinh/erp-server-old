use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, ActiveModelTrait, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  #[sea_orm(nullable)]
  pub parent_category_id: Option<Uuid>,
  pub created_at: ChronoDateTimeWithTimeZone,
  #[sea_orm(nullable)]
  pub updated_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "Entity",
    from = "Column::ParentCategoryId",
    to = "Column::Id"
  )]
  SelfReferencingCategory,
}

pub struct SelfReferencingCategory;

impl Linked for SelfReferencingCategory {
  type FromEntity = Entity;

  type ToEntity = Entity;

  fn link(&self) -> Vec<RelationDef> {
    vec![Relation::SelfReferencingCategory.def()]
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
#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult)]
#[sea_orm(entity = "Entity")]
#[serde(rename_all = "camelCase")]
pub struct PartialModel {
  pub id: Uuid,
  pub name: String,
}
