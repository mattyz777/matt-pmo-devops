use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr};
use crate::entity::release_doc;
use crate::dao::soft_delete_filter;

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<release_doc::Model>, DbErr> {
    release_doc::Entity::find_by_id(id)
        .filter(soft_delete_filter!(release_doc))
        .one(db)
        .await
}

pub async fn get_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<release_doc::Model>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    release_doc::Entity::find()
        .filter(release_doc::Column::Id.is_in(ids))
        .filter(soft_delete_filter!(release_doc))
        .all(db)
        .await
}
