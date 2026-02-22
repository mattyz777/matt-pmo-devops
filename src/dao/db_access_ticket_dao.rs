use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::db_access_ticket;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<db_access_ticket::Model>, DbErr> {
    db_access_ticket::Entity::find_by_id(id)
        .filter(soft_delete_filter!(db_access_ticket))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<db_access_ticket::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    db_access_ticket::Entity::find()
        .filter(db_access_ticket::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(db_access_ticket))
        .all(db)
        .await
}

pub async fn get_by_release_doc_id(db: &DatabaseConnection, release_doc_id: i32) -> Result<Vec<db_access_ticket::Model>, DbErr> {
    db_access_ticket::Entity::find()
        .filter(db_access_ticket::Column::ReleaseDocId.eq(release_doc_id))
        .filter(soft_delete_filter!(db_access_ticket))
        .all(db)
        .await
}
