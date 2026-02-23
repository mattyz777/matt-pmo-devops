use crate::state::AppState;
use crate::dao::release_doc as release_doc_dao;
use crate::dto::ReleaseDocDto;

pub async fn create(
    state: &AppState,
    dto: crate::dto::ReleaseDocDto,
    operator_id: i32,
) -> Result<i32, sea_orm::DbErr> {
    release_doc_dao::create(&state.db, dto, operator_id).await
}

pub async fn get(
    state: &AppState,
    id: i32,
) -> Result<Option<ReleaseDocDto>, sea_orm::DbErr> {
    // 处理数据库连接或 SQL 错误
    let model_opt = release_doc_dao::get_by_id(&state.db, id).await?;
    let dto_opt = model_opt.map(|model| model.into());
    Ok(dto_opt)
}
