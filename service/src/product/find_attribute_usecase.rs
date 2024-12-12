use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{
  attribute::{self, Entity as Attribute},
  attribute_option,
};
use infra::{util::error, uuid::Uuid};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct FindAttributeUsecase {
  pub id: Uuid,
}

pub type FindAttributeParams = FindAttributeUsecase;

#[derive(Error, Debug)]
pub enum FindAttributeError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),

  #[error("record_not_found")]
  RecordNotFound,
}

impl IntoResponse for FindAttributeError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      FindAttributeError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
      FindAttributeError::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
    };

    (status, error(code, Some("create_attribute".to_string()))).into_response()
  }
}

impl FindAttributeUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<attribute::AttributeDTO, FindAttributeError> {
    let attribute = Attribute::find_by_id(self.id)
      .find_with_related(attribute_option::Entity)
      .all(&db)
      .await?;

    let attribute_dto = attribute
      .into_iter()
      .map(|(attribute, attribute_options)| attribute::AttributeDTO {
        id: attribute.id,
        name: attribute.name,
        attribute_options: attribute_options
          .into_iter()
          .map(|option| attribute_option::PartialModel {
            id: option.id,
            value: option.value,
          })
          .collect(),
      })
      .nth(0);

    match attribute_dto {
      Some(attribute_dto) => Ok(attribute_dto),
      None => Err(FindAttributeError::RecordNotFound),
    }
  }
}
