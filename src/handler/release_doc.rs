use axum::{extract::{Path, State}, Json};
use crate::dto::{ApiResponse, ReleaseDocDto};
use crate::state::AppState;
use crate::service::release_doc as release_doc_service;

pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<ReleaseDocDto>,
) -> Json<ApiResponse<i32>> {
    // todo
    let operator_id = 1;
    match release_doc_service::create(&state, dto, operator_id).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

pub async fn get(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Json<ApiResponse<Option<ReleaseDocDto>>> {
    match release_doc_service::get(&state, id).await {
        Ok(data) => Json(ApiResponse::success(data)),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}
