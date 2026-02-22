use axum::{extract::Path, Json};

pub async fn get_user_by_id(Path(id): Path<String>) -> Json<String> {
    Json(format!("User ID: {}", id))
}
