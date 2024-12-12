use axum::{
  extract::{Query, State},
  http::StatusCode,
  Json,
};
use axum_macros::debug_handler;
use domain::product::{attribute::AttributeDTO, attribute_option};
use infra::{
  response::{CreateResponse, PaginatedResponse},
  state::AppState,
  uuid::Uuid,
};
use service::product::{
  CreateAttributeError, CreateAttributePayload, CreateAttributeUsecase,
  ListPaginatedAttributesError, ListPaginatedAttributesParams, ListPaginatedAttributesUsecase,
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

  let (attributes, pagination_meta) = usecase.invoke(state.write_db.clone()).await?;

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
