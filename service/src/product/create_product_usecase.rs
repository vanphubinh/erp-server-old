use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{
  attribute::{self},
  attribute_option, product, product_combination, product_template,
};
use infra::{util::error, uuid::Uuid};
use sea_orm::{
  prelude::Decimal, ActiveModelTrait, DbErr, EntityTrait, Set, TransactionError, TransactionTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VariantAttributeOption {
  pub attribute: attribute::PartialModel,
  pub option: attribute_option::PartialModel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
  pub price: Decimal,
  #[serde(rename(deserialize = "variantAttributeOptions"))]
  pub attribute_options: Vec<VariantAttributeOption>,
}

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
  pub variants: Vec<Variant>,
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
  ) -> Result<Vec<product::Model>, CreateProductError> {
    let payload = self.clone();

    let products = db
      .transaction::<_, Vec<product::Model>, DbErr>(move |txn| {
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
          let mut products = vec![];

          if payload.is_multiple_variants {
            for variant in payload.variants.iter() {
              let attribute_options = variant.attribute_options.iter().collect::<Vec<_>>();
              let product = product::ActiveModel {
                product_template_id: Set(product_template.id),
                price: Set(variant.price),
                cost: Set(payload.cost),
                is_product_variant: Set(true),
                ..Default::default()
              };
              let product = product.insert(txn).await?;
              let mut product_combinations = vec![];

              for option in attribute_options.iter() {
                let attribute_option = option.option.clone();
                let product_combination = product_combination::ActiveModel {
                  product_id: Set(product.id),
                  attribute_option_id: Set(attribute_option.id),
                };
                product_combinations.push(product_combination);
              }

              product_combination::Entity::insert_many(product_combinations)
                .on_empty_do_nothing()
                .exec(txn)
                .await?;

              products.push(product);
            }
          } else {
            let product = product::ActiveModel {
              product_template_id: Set(product_template.id),
              price: Set(payload.price),
              cost: Set(payload.cost),
              is_product_variant: Set(false),
              ..Default::default()
            };

            let product = product.insert(txn).await?;
            products.push(product);
          }

          Ok(products)
        })
      })
      .await?;

    Ok(products)
  }
}
