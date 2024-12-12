use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::attribute::{self, Column, Entity as Attribute};
use domain::product::attribute_option::{self, Entity as AttributeOption};
use infra::{response::PaginationMeta, util::error};
use sea_orm::{
  prelude::Expr,
  sea_query::{Alias, Asterisk, Query},
  ConnectionTrait, DbErr, EntityTrait, FromQueryResult, PaginatorTrait,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ListPaginatedAttributesUsecase {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

pub type ListPaginatedAttributesParams = ListPaginatedAttributesUsecase;

#[derive(Error, Debug)]
pub enum ListPaginatedAttributesError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedAttributesError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedAttributesError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_attributes".to_string())),
    )
      .into_response()
  }
}

impl ListPaginatedAttributesUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<(Vec<attribute::QueryResult>, PaginationMeta), ListPaginatedAttributesError> {
    let per_page = self.per_page.unwrap_or(30);
    let page = self.page.unwrap_or(1) - 1;

    let attribute_query = Query::select()
      .column((Attribute, Column::Id))
      .column((Attribute, Column::Name))
      .expr_as(
        Expr::col((AttributeOption, attribute_option::Column::Value)),
        Alias::new("attribute_option_value"),
      )
      .expr_as(
        Expr::col((AttributeOption, attribute_option::Column::Id)),
        Alias::new("attribute_option_id"),
      )
      .from_subquery(
        Query::select()
          .column(Asterisk)
          .from(Attribute)
          .limit(per_page)
          .offset(page * per_page)
          .take(),
        Alias::new("attribute"),
      )
      .left_join(
        AttributeOption,
        Expr::col((Attribute, Column::Id))
          .equals((AttributeOption, attribute_option::Column::AttributeId)),
      )
      .to_owned();

    let builder = db.get_database_backend();
    let attributes = attribute::QueryResult::find_by_statement(builder.build(&attribute_query))
      .all(&db)
      .await?;

    let total_attributes = Attribute::find().count(&db).await?;
    let total_pages = (total_attributes as f64 / per_page as f64).ceil() as u64;

    Ok((
      attributes,
      PaginationMeta {
        total: total_attributes,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }
}
