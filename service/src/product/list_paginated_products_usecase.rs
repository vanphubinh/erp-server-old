use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{
  attribute, attribute_option,
  product::{self, Column, Entity as Product},
  product_combination,
  product_template::{Column as ProductTemplateColumn, Entity as ProductTemplate},
};
use infra::{response::PaginationMeta, util::error, uuid::Uuid};
use sea_orm::{
  prelude::Expr,
  sea_query::{Alias, Query},
  ConnectionTrait, DbErr, EntityTrait, FromQueryResult, PaginatorTrait,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ListPaginatedProductsUsecase {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

pub type ListPaginatedProductsParams = ListPaginatedProductsUsecase;

#[derive(Error, Debug)]
pub enum ListPaginatedProductsError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedProductsError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedProductsError::InternalServerError(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_products".to_string())),
    )
      .into_response()
  }
}

impl ListPaginatedProductsUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<(Vec<product::ProductDTO>, PaginationMeta), ListPaginatedProductsError> {
    let per_page = self.per_page.unwrap_or(30);
    let page = self.page.unwrap_or(1) - 1;

    let product_query = Query::select()
      .column((Product, Column::Id))
      .column((Product, Column::ProductTemplateId))
      .column((Product, Column::IsProductVariant))
      .column((ProductTemplate, ProductTemplateColumn::Name))
      .expr_as(
        Expr::col((attribute::Entity, attribute::Column::Id)),
        Alias::new("attribute_id"),
      )
      .expr_as(
        Expr::col((attribute::Entity, attribute::Column::Name)),
        Alias::new("attribute_name"),
      )
      .expr_as(
        Expr::col((attribute_option::Entity, attribute_option::Column::Id)),
        Alias::new("attribute_option_id"),
      )
      .expr_as(
        Expr::col((attribute_option::Entity, attribute_option::Column::Value)),
        Alias::new("attribute_option_value"),
      )
      .from(Product)
      .left_join(
        ProductTemplate,
        Expr::col((Product, Column::ProductTemplateId))
          .equals((ProductTemplate, ProductTemplateColumn::Id)),
      )
      .left_join(
        product_combination::Entity,
        Expr::col((Product, Column::Id)).equals((
          product_combination::Entity,
          product_combination::Column::ProductId,
        )),
      )
      .left_join(
        attribute_option::Entity,
        Expr::col((
          product_combination::Entity,
          product_combination::Column::AttributeOptionId,
        ))
        .equals((attribute_option::Entity, attribute_option::Column::Id)),
      )
      .left_join(
        attribute::Entity,
        Expr::col((
          attribute_option::Entity,
          attribute_option::Column::AttributeId,
        ))
        .equals((attribute::Entity, attribute::Column::Id)),
      )
      .offset(page * per_page)
      .limit(per_page)
      .to_owned();
    let builder = db.get_database_backend();
    let product_result =
      product::QueryProductResult::find_by_statement(builder.build(&product_query))
        .all(&db)
        .await?;

    let mut product_map: std::collections::HashMap<Uuid, product::ProductDTO> =
      std::collections::HashMap::new();

    for product in product_result {
      let entry = product_map
        .entry(product.id)
        .or_insert_with(|| product::ProductDTO {
          id: product.id,
          name: product.name.clone(),
          is_product_variant: product.is_product_variant,
          combinations: Vec::new(),
        });

      if product.is_product_variant {
        let attribute_option = attribute_option::PartialModel {
          id: product.attribute_option_id.unwrap(),
          value: product.attribute_option_value.unwrap(),
        };

        if let Some(attr) = entry
          .combinations
          .iter_mut()
          .find(|attr| attr.attribute.id == product.attribute_id.unwrap())
        {
          attr.option = attribute_option;
        } else {
          entry
            .combinations
            .push(domain::product::product::AttributeWithOptionDTO {
              attribute: attribute::PartialModel {
                id: product.attribute_id.unwrap(),
                name: product.attribute_name.clone().unwrap(),
              },
              option: attribute_option,
            });
        }
      }
    }

    let products: Vec<product::ProductDTO> = product_map.into_values().collect();

    let total = Product::find().count(&db).await?;
    let total_pages = (total as f64 / per_page as f64).ceil() as u64;

    Ok((
      products,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }
}
