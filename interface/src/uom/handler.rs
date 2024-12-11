use axum::{
  extract::{Json, Path, Query, State},
  http::StatusCode,
};
use axum_macros::debug_handler;
use domain::measurement::uom::Model as Uom;
use infra::{
  response::{CreateResponse, FindOneResponse, PaginatedResponse, UpdateResponse},
  state::AppState,
};
use service::measurement::{
  CreateUomError, CreateUomParams, CreateUomUsecase, FindUomError, FindUomParams, FindUomUsecase,
  ListPaginatedUomsError, ListPaginatedUomsParams, ListPaginatedUomsUsecase, UpdateUomError,
  UpdateUomParams, UpdateUomUsecase,
};
use std::sync::Arc;

#[debug_handler]
pub async fn list_paginated_uoms(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedUomsParams>,
) -> Result<PaginatedResponse<Uom>, ListPaginatedUomsError> {
  let usecase = ListPaginatedUomsUsecase {
    page: Some(query.page.unwrap_or(1)),
    per_page: Some(query.per_page.unwrap_or(30)),
  };

  let (uoms, meta) = usecase.invoke(state.read_db.clone()).await?;

  Ok(PaginatedResponse::<Uom> {
    ok: true,
    data: uoms,
    meta,
  })
}

#[debug_handler]
pub async fn create_uom(
  State(state): State<Arc<AppState>>,
  Json(body): Json<CreateUomParams>,
) -> Result<(StatusCode, CreateResponse), CreateUomError> {
  let usecase = CreateUomUsecase { name: body.name };

  let uom = usecase.invoke(state.read_db.clone()).await?;

  Ok((
    StatusCode::CREATED,
    CreateResponse {
      id: uom.id,
      ok: true,
    },
  ))
}

#[debug_handler]
pub async fn find_uom(
  State(state): State<Arc<AppState>>,
  Path(path): Path<FindUomParams>,
) -> Result<FindOneResponse<Uom>, FindUomError> {
  let usecase = FindUomUsecase { id: path.id };

  let uom = usecase.invoke(state.read_db.clone()).await?;

  Ok(FindOneResponse::<Uom> {
    ok: true,
    data: uom,
  })
}

#[debug_handler]
pub async fn update_uom(
  State(state): State<Arc<AppState>>,
  Json(body): Json<UpdateUomParams>,
) -> Result<(StatusCode, UpdateResponse<Uom>), UpdateUomError> {
  let usecase = UpdateUomUsecase {
    id: body.id,
    name: body.name,
  };

  let uom = usecase.invoke(state.read_db.clone()).await?;

  Ok((
    StatusCode::OK,
    UpdateResponse::<Uom> {
      data: uom,
      ok: true,
    },
  ))
}
