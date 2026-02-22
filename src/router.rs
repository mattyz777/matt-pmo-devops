use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use crate::handler::{common, user};

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        // GET http://localhost:3000/    - Hello World
        .route("/", get(|| async { "Hello, World!" }))
        // GET  http://localhost:3000/foo - "get foo"
        // POST http://localhost:3000/foo - {"data": 42}
        .route("/foo", get(common::get_foo).post(common::post_foo))
        // GET http://localhost:3000/users/1 - 获取用户信息
        .route("/users/{id}", get(user::get_user_by_id))
        .with_state(db)
}
