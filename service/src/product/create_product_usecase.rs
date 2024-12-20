use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{product, product_template};
use infra::{util::error, uuid::Uuid};
use sea_orm::{prelude::Decimal, ActiveModelTrait, DbErr, Set, TransactionError, TransactionTrait};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProductUsecase {
  pub name: String,
  #[serde(rename(deserialize = "productType"))]
  pub product_type: product_template::ProductType,
  #[serde(rename(deserialize = "productSubtype"))]
  pub product_subtype: product_template::ProductSubtype,
  #[serde(rename(deserialize = "isTrackInventory"))]
  pub is_track_inventory: bool,
  pub price: Decimal,
  pub cost: Decimal,
  #[serde(rename(deserialize = "uomId"))]
  pub uom_id: Uuid,
  #[serde(rename(deserialize = "categoryId"))]
  pub category_id: Option<Uuid>,
  #[serde(rename(deserialize = "createCorrespondingMoulds"))]
  pub create_corresponding_moulds: bool,
  #[serde(rename(deserialize = "isMultipleVariants"))]
  pub is_multiple_variants: bool,
}

pub type CreateProductPayload = CreateProductUsecase;

#[derive(thiserror::Error, Debug)]
pub enum CreateProductError {
  #[error("internal_server_error")]
  InternalServerError(#[from] TransactionError<DbErr>),
}

impl IntoResponse for CreateProductError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      CreateProductError::InternalServerError(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
      }
    };

    (status, error(code, Some("create_uom".to_string()))).into_response()
  }
}

impl CreateProductUsecase {
  pub async fn invoke(
    &self,
    db: impl TransactionTrait,
  ) -> Result<product::Model, CreateProductError> {
    let payload = self.clone();

    let product = db
      .transaction::<_, product::Model, DbErr>(move |txn| {
        Box::pin(async move {
          let product_template = product_template::ActiveModel {
            name: Set(payload.name),
            product_type: Set(payload.product_type),
            product_subtype: Set(payload.product_subtype),
            is_track_inventory: Set(payload.is_track_inventory),
            uom_id: Set(payload.uom_id),
            category_id: Set(payload.category_id),
            ..Default::default()
          };
          let product_template = product_template.insert(txn).await?;

          let product = product::ActiveModel {
            product_template_id: Set(product_template.id),
            price: Set(payload.price),
            cost: Set(payload.cost),
            ..Default::default()
          };

          let product = product.insert(txn).await?;

          Ok(product)
        })
      })
      .await?;

    Ok(product)
  }
}
