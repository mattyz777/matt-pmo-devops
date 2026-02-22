use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::feature;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<feature::Model>, DbErr> {
    feature::Entity::find_by_id(id)
        .filter(soft_delete_filter!(feature))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<feature::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    feature::Entity::find()
        .filter(feature::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(feature))
        .all(db)
        .await
}

pub async fn get_by_release_note_id(db: &DatabaseConnection, release_note_id: i32) -> Result<Vec<feature::Model>, DbErr> {
    feature::Entity::find()
        .filter(feature::Column::ReleaseNoteId.eq(release_note_id))
        .filter(soft_delete_filter!(feature))
        .all(db)
        .await
}
