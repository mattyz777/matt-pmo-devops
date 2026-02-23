use sea_orm::DatabaseConnection;
use crate::dto::ApiResponse;

/// 根据ID查询
pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> ApiResponse<crate::dto::ReleaseDocDto> {
    match crate::dao::release_doc::get_by_id(db, id).await {
        Ok(Some(entity)) => ApiResponse::success(entity.into()),
        Ok(None) => ApiResponse::<crate::dto::ReleaseDocDto> {
            code: 0,
            data: None,
            message: "success".to_string(),
        },
        Err(_) => ApiResponse::error("Internal server error".to_string()),
    }
}

/// 创建 ReleaseDoc
pub async fn create(
    db: &DatabaseConnection,
    dto: crate::dto::ReleaseDocDto,
    operator_id: i32,
) -> ApiResponse<crate::dto::ReleaseDocDto> {
    match crate::dao::release_doc::create(db, dto, operator_id).await {
        Ok(id) => {
            // 创建成功，再查询返回完整数据
            match crate::dao::release_doc::get_by_id(db, id).await {
                Ok(Some(entity)) => ApiResponse::success(entity.into()),
                Ok(None) => ApiResponse::error("Failed to fetch created record".to_string()),
                Err(_) => ApiResponse::error("Internal server error".to_string()),
            }
        }
        Err(_) => ApiResponse::error("Failed to create release document".to_string()),
    }
}

/// 更新 ReleaseDoc
pub async fn update(
    db: &DatabaseConnection,
    dto: crate::dto::ReleaseDocDto,
    operator_id: i32,
) -> ApiResponse<crate::dto::ReleaseDocDto> {
    match crate::dao::release_doc::update(db, dto.clone(), operator_id).await {
        Ok(entity) => ApiResponse::success(entity.into()),
        Err(e) => {
            if matches!(e, sea_orm::DbErr::RecordNotFound(_)) {
                ApiResponse::error(format!("ReleaseDoc not found"))
            } else {
                ApiResponse::error("Failed to update release document".to_string())
            }
        }
    }
}

/// 设置准备状态
pub async fn set_ready(
    db: &DatabaseConnection,
    id: i32,
    is_ready: bool,
    operator_id: i32,
) -> ApiResponse<()> {
    match crate::dao::release_doc::set_ready(db, id, is_ready, operator_id).await {
        Ok(()) => ApiResponse { code: 0, data: None, message: "success".to_string() },
        Err(_) => ApiResponse::error("Failed to update status".to_string()),
    }
}

/// 列出所有 ReleaseDoc
pub async fn list(
    db: &DatabaseConnection,
) -> ApiResponse<Vec<crate::dto::ReleaseDocDto>> {
    match crate::dao::release_doc::list(db).await {
        Ok(entities) => {
            let dtos: Vec<crate::dto::ReleaseDocDto> = entities.into_iter().map(|e| e.into()).collect();
            ApiResponse::success(dtos)
        }
        Err(_) => ApiResponse::error("Failed to fetch release documents".to_string()),
    }
}

/// 删除 ReleaseDoc
pub async fn delete(
    db: &DatabaseConnection,
    id: i32,
    operator_id: i32,
) -> ApiResponse<()> {
    match crate::dao::release_doc::delete(db, id, operator_id).await {
        Ok(()) => ApiResponse { code: 0, data: None, message: "success".to_string() },
        Err(_) => ApiResponse::error("Failed to delete release document".to_string()),
    }
}
