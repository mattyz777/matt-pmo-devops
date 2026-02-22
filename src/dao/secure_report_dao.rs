use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::secure_report;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<secure_report::Model>, DbErr> {
    secure_report::Entity::find_by_id(id)
        .filter(soft_delete_filter!(secure_report))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<secure_report::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    secure_report::Entity::find()
        .filter(secure_report::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(secure_report))
        .all(db)
        .await
}

pub async fn get_by_release_note_id(db: &DatabaseConnection, release_note_id: i32) -> Result<Vec<secure_report::Model>, DbErr> {
    secure_report::Entity::find()
        .filter(secure_report::Column::ReleaseNoteId.eq(release_note_id))
        .filter(soft_delete_filter!(secure_report))
        .all(db)
        .await
}
