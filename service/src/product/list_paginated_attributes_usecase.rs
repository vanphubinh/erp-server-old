use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::attribute::{self, Entity as Attribute};
use infra::{response::PaginationMeta, util::error};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait};
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
  ) -> Result<(Vec<attribute::PartialModel>, PaginationMeta), ListPaginatedAttributesError> {
    let per_page = self.per_page.unwrap_or(30);
    let page = self.page.unwrap_or(1) - 1;

    let attribute_pages = Attribute::find()
      .into_partial_model::<attribute::PartialModel>()
      .paginate(&db, per_page);
    let attributes = attribute_pages.fetch_page(page).await?;
    let items_and_pages = attribute_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      attributes,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }
}
