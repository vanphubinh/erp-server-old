use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::{
  product::{self, Column, Entity as Product},
  product_template::{Column as ProductTemplateColumn, Entity as ProductTemplate},
};
use infra::{response::PaginationMeta, util::error};
use sea_orm::{
  prelude::Expr, sea_query::Query, ConnectionTrait, DbErr, EntityTrait, FromQueryResult,
  PaginatorTrait,
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
  ) -> Result<(Vec<product::QueryProductResult>, PaginationMeta), ListPaginatedProductsError> {
    let per_page = self.per_page.unwrap_or(30);
    let page = self.page.unwrap_or(1) - 1;

    let product_query = Query::select()
      .column((Product, Column::Id))
      .column((Product, Column::ProductTemplateId))
      .column((ProductTemplate, ProductTemplateColumn::Name))
      .from(Product)
      .left_join(
        ProductTemplate,
        Expr::col((Product, Column::ProductTemplateId))
          .equals((ProductTemplate, ProductTemplateColumn::Id)),
      )
      .to_owned();
    let builder = db.get_database_backend();
    let products = product::QueryProductResult::find_by_statement(builder.build(&product_query))
      .all(&db)
      .await?;

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
