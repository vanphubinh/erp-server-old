use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::{self, ActiveModel as Uom};
use infra::{util::error, uuid::Uuid};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct UpdateUomUsecase {
  pub id: Uuid,
  pub name: String,
}

pub type UpdateUomParams = UpdateUomUsecase;

#[derive(Error, Debug)]
pub enum UpdateUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for UpdateUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      UpdateUomError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("create_uom".to_string()))).into_response()
  }
}

impl UpdateUomUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<uom::PartialModel, UpdateUomError> {
    let uom = Uom {
      id: Set(self.id),
      name: Set(self.name.to_string()),
      ..Default::default()
    };
    let updated_uom = uom.update(&db).await?;
    let partial_uom = uom::PartialModel {
      id: updated_uom.id,
      name: updated_uom.name,
    };

    Ok(partial_uom)
  }
}
