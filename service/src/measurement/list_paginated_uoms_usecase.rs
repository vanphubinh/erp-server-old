use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use domain::measurement::uom::{self, Column, Entity as Uom};
use infra::{response::PaginationMeta, util::error};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QuerySelect};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ListPaginatedUomsUsecase {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

pub type ListPaginatedUomsParams = ListPaginatedUomsUsecase;

#[derive(Error, Debug)]
pub enum ListPaginatedUomsError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListPaginatedUomsError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListPaginatedUomsError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (status, error(code, Some("list_paginated_uoms".to_string()))).into_response()
  }
}

impl ListPaginatedUomsUsecase {
  pub async fn invoke(
    &self,
    db: impl ConnectionTrait,
  ) -> Result<(Vec<uom::Model>, PaginationMeta), ListPaginatedUomsError> {
    let per_page = self.per_page.unwrap_or(30);
    let page = self.page.unwrap_or(1) - 1;

    let uom_pages = Uom::find()
      .select_only()
      .column(Column::Id)
      .column(Column::Name)
      .paginate(&db, per_page);
    let uoms = uom_pages.fetch_page(page).await?;
    let items_and_pages = uom_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
      uoms,
      PaginationMeta {
        total,
        total_pages,
        page: page + 1,
        per_page,
      },
    ))
  }
}
