use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::release_doc;
use crate::dto::ReleaseDocDto;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<release_doc::Model>, DbErr> {
    release_doc::Entity::find_by_id(id)
        .filter(release_doc::Column::IsDelete.eq(false))
        .one(db)
        .await
}

pub async fn list(db: &DatabaseConnection) -> Result<Vec<release_doc::Model>, DbErr> {
    release_doc::Entity::find()
        .filter(release_doc::Column::IsDelete.eq(false))
        .all(db)
        .await
}

pub async fn create(db: &DatabaseConnection, dto: ReleaseDocDto, operator_id: i32) -> Result<i32, DbErr> {
    let active_model = dto.into_create_model(operator_id);
    let result = release_doc::Entity::insert(active_model)
        .exec(db)
        .await?;

    Ok(result.last_insert_id)
}

pub async fn update(db: &DatabaseConnection, dto: ReleaseDocDto, operator_id: i32) -> Result<release_doc::Model, DbErr> {
    let active_model = dto.into_update_model(operator_id);
    active_model.update(db).await
}

pub async fn set_ready(db: &DatabaseConnection, id: i32, is_ready: bool, operator_id: i32) -> Result<(), DbErr> {
    let now = chrono::Utc::now();

    let active_model = release_doc::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        is_ready: sea_orm::ActiveValue::Set(is_ready),
        updated_at: sea_orm::ActiveValue::Set(Some(now)),
        updator: sea_orm::ActiveValue::Set(Some(operator_id)),
        ..Default::default()
    };

    active_model.update(db).await?;
    Ok(())
}

pub async fn delete(db: &DatabaseConnection, id: i32, operator_id: i32) -> Result<(), DbErr> {
    let now = chrono::Utc::now();

    let active_model = release_doc::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        is_delete: sea_orm::ActiveValue::Set(true),
        updated_at: sea_orm::ActiveValue::Set(Some(now)),
        updator: sea_orm::ActiveValue::Set(Some(operator_id)),
        ..Default::default()
    };

    active_model.update(db).await?;
    Ok(())
}
