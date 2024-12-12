use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::product::category::{self, Entity as Category};
use infra::{response::PaginationMeta, util::error};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ListPaginatedCategoriesUsecase {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

pub type ListPaginatedCategoriesParams = ListPaginatedCategoriesUsecase;

#[derive(Error, Debug)]
pub enum ListPaginatedCategoriesError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedCategoriesError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedCategoriesError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_categories".to_string())),
    )
      .into_response()
  }
}

impl ListPaginatedCategoriesUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<(Vec<category::PartialModel>, PaginationMeta), ListPaginatedCategoriesError> {
    let per_page = self.per_page.unwrap_or(30);
    let page = self.page.unwrap_or(1) - 1;

    let category_pages = Category::find()
      .into_partial_model::<category::PartialModel>()
      .paginate(&db, per_page);
    let categories = category_pages.fetch_page(page).await?;
    let items_and_pages = category_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      categories,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }
}
