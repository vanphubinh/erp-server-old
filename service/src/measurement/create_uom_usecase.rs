use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::{self, ActiveModel as Uom};
use infra::util::error;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct CreateUomUsecase {
  pub name: String,
}

pub type CreateUomParams = CreateUomUsecase;

#[derive(Error, Debug)]
pub enum CreateUomError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for CreateUomError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateUomError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("create_uom".to_string()))).into_response()
  }
}

impl CreateUomUsecase {
  pub async fn invoke(&self, db: impl ConnectionTrait) -> Result<uom::Model, CreateUomError> {
    let uom = Uom {
      name: Set(self.name.to_owned()),
      ..Default::default()
    };
    let uom = uom.insert(&db).await?;

    Ok(uom)
  }
}
