use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{
  attribute::{self, ActiveModel as Attribute},
  attribute_option,
};
use infra::{util::error, uuid::Uuid};
use sea_orm::{
  sea_query::OnConflict, ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set,
  TransactionError, TransactionTrait,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateAttributeUsecase {
  pub id: Uuid,
  pub name: String,
  #[serde(rename(deserialize = "attributeOptions"))]
  pub attribute_options: Vec<AttributeOption>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AttributeOption {
  pub id: Option<Uuid>,
  pub value: String,
}

pub type UpdateAttributePayload = UpdateAttributeUsecase;

#[derive(Error, Debug)]
pub enum UpdateAttributeError {
  #[error("internal_server_error")]
  InternalServerError(#[from] TransactionError<DbErr>),
}

impl IntoResponse for UpdateAttributeError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      UpdateAttributeError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("create_attribute".to_string()))).into_response()
  }
}

impl UpdateAttributeUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait + TransactionTrait,
  ) -> Result<attribute::Model, UpdateAttributeError> {
    let payload = self.clone();

    let attribute = db
      .transaction::<_, attribute::Model, DbErr>(move |txn| {
        Box::pin(async move {
          let id = payload.id;
          let name = payload.name.to_string();
          let attribute = Attribute {
            id: Set(id),
            name: Set(name),
            ..Default::default()
          };
          let attribute = attribute.update(txn).await?;

          let options =
            payload
              .attribute_options
              .into_iter()
              .map(|option| attribute_option::ActiveModel {
                id: Set(option.id.unwrap_or(Uuid::new())),
                value: Set(option.value.to_string()),
                attribute_id: Set(id),
              });

          attribute_option::Entity::insert_many(options)
            .on_conflict(
              OnConflict::columns([attribute_option::Column::Id])
                .update_column(attribute_option::Column::Value)
                .to_owned(),
            )
            .exec(txn)
            .await?;
          Ok(attribute)
        })
      })
      .await?;

    Ok(attribute)
  }
}
