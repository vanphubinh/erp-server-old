use axum::{
  extract::{Path, Query, State},
  http::StatusCode,
  Json,
};
use axum_macros::debug_handler;
use domain::product::{attribute::AttributeDTO, attribute_option};
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
use std::{collections::HashMap, sync::Arc};

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
) -> Result<Json<PaginatedResponse<AttributeDTO>>, ListPaginatedAttributesError> {
  let usecase = ListPaginatedAttributesUsecase {
    page: query.page,
    per_page: query.per_page,
  };

  let (attributes, pagination_meta) = usecase.invoke(state.read_db.clone()).await?;

  let mut attribute_map: HashMap<Uuid, AttributeDTO> = HashMap::new();

  for (_, attribute) in attributes.into_iter().enumerate() {
    let entry = attribute_map.entry(attribute.id).or_insert(AttributeDTO {
      id: attribute.id,
      name: attribute.name.clone(),
      attribute_options: Vec::new(),
    });

    if let (Some(attribute_option_id), Some(attribute_option_value)) = (
      attribute.attribute_option_id,
      attribute.attribute_option_value,
    ) {
      entry
        .attribute_options
        .push(attribute_option::PartialModel {
          id: attribute_option_id,
          value: attribute_option_value,
        });
    }
  }

  let attributes_with_options: Vec<AttributeDTO> = attribute_map.into_values().collect();

  Ok(Json(PaginatedResponse {
    data: attributes_with_options,
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
