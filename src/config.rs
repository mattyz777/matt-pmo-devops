use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url).await?;
    Ok(db)
}