use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::{self, Entity as Uom};
use infra::{util::error, uuid::Uuid};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct FindUomUsecase {
  pub id: Uuid,
}

pub type FindUomParams = FindUomUsecase;

#[derive(Error, Debug)]
pub enum FindUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),

  #[error("record_not_found")]
  RecordNotFound,
}

impl IntoResponse for FindUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      FindUomError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
      FindUomError::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
    };

    (status, error(code, Some("create_uom".to_string()))).into_response()
  }
}

impl FindUomUsecase {
  pub async fn invoke(&self, db: impl ConnectionTrait) -> Result<uom::PartialModel, FindUomError> {
    let uom = Uom::find_by_id(self.id)
      .into_partial_model::<uom::PartialModel>()
      .one(&db)
      .await?;

    match uom {
      Some(uom) => Ok(uom),
      None => Err(FindUomError::RecordNotFound),
    }
  }
}
