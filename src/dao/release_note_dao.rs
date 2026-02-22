use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::release_note;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<release_note::Model>, DbErr> {
    release_note::Entity::find_by_id(id)
        .filter(soft_delete_filter!(release_note))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<release_note::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    release_note::Entity::find()
        .filter(release_note::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(release_note))
        .all(db)
        .await
}

pub async fn get_by_release_doc_id(db: &DatabaseConnection, release_doc_id: i32) -> Result<Vec<release_note::Model>, DbErr> {
    release_note::Entity::find()
        .filter(release_note::Column::ReleaseDocId.eq(release_doc_id))
        .filter(soft_delete_filter!(release_note))
        .all(db)
        .await
}
