use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, DbErr, QueryOrder, PaginatorTrait};
use crate::entity::user;

const DEFAULT_PAGE_SIZE: u64 = 20;


pub async fn create(db: &DatabaseConnection, active_model: user::ActiveModel) -> Result<i32, DbErr> {
    let result = user::Entity::insert(active_model)
        .exec(db)
        .await?;
    Ok(result.last_insert_id)
}


pub async fn update(db: &DatabaseConnection, active_model: user::ActiveModel) -> Result<(), DbErr> {
    active_model.update(db).await?;
    Ok(())
}


pub async fn delete(db: &DatabaseConnection, id: i32, operator_id: i32) -> Result<(), DbErr> {
    let now = chrono::Utc::now();

    let active_model = user::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        is_deleted: sea_orm::ActiveValue::Set(true),
        updated_at: sea_orm::ActiveValue::Set(Some(now)),
        updated_by: sea_orm::ActiveValue::Set(Some(operator_id)),
        ..Default::default()
    };

    active_model.update(db).await?;
    Ok(())
}


pub async fn list(
    db: &DatabaseConnection,
    page: Option<u64>,
    page_size: Option<u64>,
) -> Result<(Vec<user::Model>, u64), DbErr> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    let paginator = user::Entity::find()
        .filter(user::Column::IsDeleted.eq(false))
        .order_by_desc(user::Column::CreatedAt)
        .paginate(db, page_size);

    let total = paginator.num_pages().await?;
    let models = paginator.fetch_page(page - 1).await?;

    Ok((models, total))
}


pub async fn get_by_id(db: &DatabaseConnection, record_id: i32) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(record_id)
        .filter(user::Column::IsDeleted.eq(false))
        .one(db)
        .await
}


pub async fn get_by_username(db: &DatabaseConnection, username: &str) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .filter(user::Column::IsDeleted.eq(false))
        .one(db)
        .await
}
