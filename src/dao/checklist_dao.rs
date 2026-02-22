use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::checklist;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<checklist::Model>, DbErr> {
    checklist::Entity::find_by_id(id)
        .filter(soft_delete_filter!(checklist))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<checklist::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    checklist::Entity::find()
        .filter(checklist::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(checklist))
        .all(db)
        .await
}

pub async fn get_by_release_doc_id(db: &DatabaseConnection, release_doc_id: i32) -> Result<Vec<checklist::Model>, DbErr> {
    checklist::Entity::find()
        .filter(checklist::Column::ReleaseDocId.eq(release_doc_id))
        .filter(soft_delete_filter!(checklist))
        .all(db)
        .await
}
