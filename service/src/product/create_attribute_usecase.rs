use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{
  attribute::{self, ActiveModel as Attribute},
  attribute_option,
};
use infra::util::error;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, Set, TransactionError, TransactionTrait};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct CreateAttributeUsecase {
  pub name: String,
  pub attribute_options: Vec<AttributeOption>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct AttributeOption {
  pub value: String,
}
pub type CreateAttributePayload = CreateAttributeUsecase;

#[derive(Error, Debug)]
pub enum CreateAttributeError {
  #[error("internal_server_error")]
  InternalServerError(#[from] TransactionError<DbErr>),
}

impl IntoResponse for CreateAttributeError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateAttributeError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("create_uom".to_string()))).into_response()
  }
}

impl CreateAttributeUsecase {
  pub async fn invoke(
    &self,
    db: impl TransactionTrait,
  ) -> Result<attribute::Model, CreateAttributeError> {
    let name = self.name.to_owned();
    let attribute_options = self.attribute_options.to_owned();
    let attribute = db
      .transaction::<_, attribute::Model, DbErr>(move |txn| {
        Box::pin(async move {
          let attribute = Attribute {
            name: Set(name),
            ..Default::default()
          };
          let attribute = attribute.insert(txn).await?;
          let mut options: Vec<attribute_option::ActiveModel> = Vec::new();
          for (_, option) in attribute_options.into_iter().enumerate() {
            let attribute_option = attribute_option::ActiveModel {
              value: Set(option.value.to_string()),
              attribute_id: Set(attribute.id),
              ..Default::default()
            };
            options.push(attribute_option);
          }
          if !options.is_empty() {
            attribute_option::Entity::insert_many(options)
              .exec(txn)
              .await?;
          }

          Ok(attribute)
        })
      })
      .await?;

    Ok(attribute)
  }
}
