use crate::state::AppState;
use crate::dao::release_doc as release_doc_dao;
use crate::dto::{ReleaseDocRequestDto, ReleaseDocResponseDto};

pub async fn create(
    state: &AppState,
    dto: ReleaseDocRequestDto,
    operator_id: i32,
) -> Result<i32, sea_orm::DbErr> {
    release_doc_dao::create(&state.db, dto, operator_id).await
}

pub async fn get(
    state: &AppState,
    record_id: i32,
) -> Result<Option<ReleaseDocResponseDto>, sea_orm::DbErr> {
    let model_opt = release_doc_dao::get_by_id(&state.db, record_id).await?;
    let dto_opt = model_opt.map(|model| model.into());
    Ok(dto_opt)
}


pub async fn update(
    state: &AppState,
    record_id: i32,
    dto: ReleaseDocRequestDto,
    operator_id: i32,
) -> Result<(), sea_orm::DbErr> {
    release_doc_dao::update(&state.db, dto, record_id, operator_id).await?;
    Ok(())
}