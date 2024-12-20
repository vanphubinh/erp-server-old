use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::attribute_option;
use infra::{util::error, uuid::Uuid};
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct FindOptionsByAttributeIdUsecase {
  pub attribute_id: Uuid,
}

pub type FindOptionsByAttributeIdParams = FindOptionsByAttributeIdUsecase;

#[derive(Error, Debug)]
pub enum FindOptionsByAttributeIdError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),

  #[error("record_not_found")]
  RecordNotFound,
}

impl IntoResponse for FindOptionsByAttributeIdError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      FindOptionsByAttributeIdError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
      FindOptionsByAttributeIdError::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
    };

    (status, error(code, Some("create_attribute".to_string()))).into_response()
  }
}

impl FindOptionsByAttributeIdUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<Vec<attribute_option::PartialModel>, FindOptionsByAttributeIdError> {
    let options = attribute_option::Entity::find()
      .filter(attribute_option::Column::AttributeId.eq(self.attribute_id))
      .into_partial_model::<attribute_option::PartialModel>()
      .all(&db)
      .await?;

    Ok(options)
  }
}
