use axum::{
  extract::{Path, Query, State},
  http::StatusCode,
  Json,
};
use axum_macros::debug_handler;
use domain::product::{
  attribute::{self, AttributeDTO},
  attribute_option,
};
use infra::{
  response::{CreateResponse, FindOneResponse, OkResponse, PaginatedResponse, QueryResponse},
  state::AppState,
  uuid::Uuid,
};
use service::product::{
  update_attribute_usecase::{UpdateAttributeError, UpdateAttributeUsecase},
  CreateAttributeError, CreateAttributePayload, CreateAttributeUsecase, FindAttributeError,
  FindAttributeUsecase, FindOptionsByAttributeIdError, FindOptionsByAttributeIdUsecase,
  ListPaginatedAttributesError, ListPaginatedAttributesParams, ListPaginatedAttributesUsecase,
  UpdateAttributePayload,
};
use std::sync::Arc;

#[debug_handler]
pub async fn create_attribute(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateAttributePayload>,
) -> Result<(StatusCode, CreateResponse), CreateAttributeError> {
  let usecase = CreateAttributeUsecase {
    name: payload.name,
    attribute_options: payload.attribute_options,
  };

  let created_attribute = usecase.invoke(state.write_db.clone()).await?;

  Ok((
    StatusCode::CREATED,
    CreateResponse {
      id: created_attribute.id,
      ok: true,
    },
  ))
}

#[debug_handler]
pub async fn list_paginated_attributes(
  State(state): State<Arc<AppState>>,
  Query(query): Query<ListPaginatedAttributesParams>,
) -> Result<Json<PaginatedResponse<attribute::PartialModel>>, ListPaginatedAttributesError> {
  let usecase = ListPaginatedAttributesUsecase {
    page: query.page,
    per_page: query.per_page,
  };

  let (attributes, pagination_meta) = usecase.invoke(state.read_db.clone()).await?;

  Ok(Json(PaginatedResponse {
    data: attributes,
    meta: pagination_meta,
    ok: true,
  }))
}

#[debug_handler]
pub async fn find_attribute(
  State(state): State<Arc<AppState>>,
  Path(id): Path<Uuid>,
) -> Result<FindOneResponse<AttributeDTO>, FindAttributeError> {
  let usecase = FindAttributeUsecase { id };
  let attribute = usecase.invoke(state.read_db.clone()).await?;
  Ok(FindOneResponse::<AttributeDTO> {
    ok: true,
    data: attribute,
  })
}

#[debug_handler]
pub async fn update_attribute(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<UpdateAttributePayload>,
) -> Result<OkResponse, UpdateAttributeError> {
  let usecase = UpdateAttributeUsecase {
    id: payload.id,
    name: payload.name,
    attribute_options: payload.attribute_options,
  };
  usecase.invoke(state.read_db.clone()).await?;
  Ok(OkResponse { ok: true })
}

#[debug_handler]
pub async fn find_options_by_attribute_id(
  State(state): State<Arc<AppState>>,
  Path(attribute_id): Path<Uuid>,
) -> Result<QueryResponse<Vec<attribute_option::PartialModel>>, FindOptionsByAttributeIdError> {
  let usecase = FindOptionsByAttributeIdUsecase { attribute_id };
  let options = usecase.invoke(state.read_db.clone()).await?;
  Ok(QueryResponse::<Vec<attribute_option::PartialModel>> {
    ok: true,
    data: options,
  })
}
