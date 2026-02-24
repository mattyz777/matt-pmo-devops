use axum::{extract::{Path, State}, Json};
use crate::dto::{ApiResponse, ReleaseDocDto};
use crate::state::AppState;
use crate::service::release_doc as release_doc_service;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<ReleaseDocDto>,
) -> ApiResponse<i32> {
    // todo
    let operator_id = 1;
    match release_doc_service::create(&state, dto, operator_id).await {
        Ok(id) => ApiResponse::success(id),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

pub async fn get(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> ApiResponse<Option<ReleaseDocDto>> {
    match release_doc_service::get(&state, id).await {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

pub async fn update(
    Path(id): Path<i32>,
    Json(dto): Json<ReleaseDocDto>,
    State(state): State<AppState>,
) -> ApiResponse<()> {
    // todo
    let operator_id = 1;
    match release_doc_service::update(&state, id, dto, operator_id).await {
        Ok(()) => ApiResponse::<()>::success_without_data(),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}