use axum::{extract::{Path, State}, Json};
use crate::dto::{ApiResponse, PageResult, UserRequestDto, UserResponseDto, UserUpdateDto, UserListQueryDto};
use crate::state::AppState;
use crate::service::user as user_service;


pub async fn create(
    State(state): State<AppState>,
    Json(dto): Json<UserRequestDto>,
) -> ApiResponse<i32> {
    // todo: get operator_id from auth context
    let operator_id = 1;
    match user_service::create(&state, dto, operator_id).await {
        Ok(id) => ApiResponse::success(id),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}


pub async fn update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(dto): Json<UserUpdateDto>,
) -> ApiResponse<()> {
    // todo: get operator_id from auth context
    let operator_id = 1;
    match user_service::update(&state, id, dto, operator_id).await {
        Ok(()) => ApiResponse::<()>::success_without_data(),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}


pub async fn get_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> ApiResponse<Option<UserResponseDto>> {
    match user_service::get_by_id(&state, id).await {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}


pub async fn list(
    State(state): State<AppState>,
    Json(query): Json<UserListQueryDto>,
) -> ApiResponse<PageResult<UserResponseDto>> {
    match user_service::list(&state, query.page, query.page_size).await {
        Ok(data) => ApiResponse::success(data),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}



