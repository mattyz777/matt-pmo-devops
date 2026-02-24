use crate::state::AppState;
use crate::dao::user as user_dao;
use crate::dto::{
    UserRequestDto, UserUpdateDto, UserResponseDto, UserListQueryDto, UserListResponseDto,
};
use crate::utils::encrypt::hash_password;


pub async fn create(
    state: &AppState,
    dto: UserRequestDto,
    operator_id: i32,
) -> Result<i32, sea_orm::DbErr> {
    let hashed_password = hash_password(&dto.password)
        .map_err(|e| sea_orm::DbErr::Custom(format!("Password hash error: {}", e)))?;

    let active_model = dto.into_create_model(operator_id, hashed_password);
    user_dao::create(&state.db, active_model).await
}


pub async fn get_by_id(
    state: &AppState,
    id: i32,
) -> Result<Option<UserResponseDto>, sea_orm::DbErr> {
    let model_opt = user_dao::get_by_id(&state.db, id).await?;
    let dto_opt = model_opt.map(|model| model.into());
    Ok(dto_opt)
}


pub async fn get_by_username(
    state: &AppState,
    username: &str,
) -> Result<Option<UserResponseDto>, sea_orm::DbErr> {
    let model_opt = user_dao::get_by_username(&state.db, username).await?;
    let dto_opt = model_opt.map(|model| model.into());
    Ok(dto_opt)
}


pub async fn list(
    state: &AppState,
    query: UserListQueryDto,
) -> Result<UserListResponseDto, sea_orm::DbErr> {
    let (models, total) = user_dao::list(&state.db, query.page, query.page_size).await?;
    let data: Vec<UserResponseDto> = models.into_iter().map(|m| m.into()).collect();

    Ok(UserListResponseDto {
        data,
        total,
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(20),
    })
}


/// username 不能修改
pub async fn update(
    state: &AppState,
    id: i32,
    dto: UserUpdateDto,
    operator_id: i32,
) -> Result<(), sea_orm::DbErr> {
    let user_model = user_dao::get_by_id(&state.db, id).await?
        .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("User {} not found", id)))?;

    let hashed_password = if let Some(ref password) = dto.password {
        Some(hash_password(password)
            .map_err(|e| sea_orm::DbErr::Custom(format!("Hash error: {}", e)))?)
    } else {
        None
    };

    let active_model = dto.into_update_model(user_model.into(), operator_id, hashed_password);
    user_dao::update(&state.db, active_model).await
}


pub async fn delete(state: &AppState, id: i32, operator_id: i32) -> Result<(), sea_orm::DbErr> {
    user_dao::delete(&state.db, id, operator_id).await
}
