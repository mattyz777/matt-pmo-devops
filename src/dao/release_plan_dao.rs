use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::release_plan;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<release_plan::Model>, DbErr> {
    release_plan::Entity::find_by_id(id)
        .filter(soft_delete_filter!(release_plan))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<release_plan::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    release_plan::Entity::find()
        .filter(release_plan::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(release_plan))
        .all(db)
        .await
}

pub async fn get_by_release_doc_id(db: &DatabaseConnection, release_doc_id: i32) -> Result<Vec<release_plan::Model>, DbErr> {
    release_plan::Entity::find()
        .filter(release_plan::Column::ReleaseDocId.eq(release_doc_id))
        .filter(soft_delete_filter!(release_plan))
        .all(db)
        .await
}
