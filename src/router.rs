use axum::{Router, routing::{get, post}};
use crate::handler::{common, user, release_doc};
use crate::state::AppState;

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(common::get_foo).post(common::post_foo))
        // business routes
        .nest("/api", api_routes())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_routes())
        .nest("/release-doc", release_doc_routes())
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(user::get_user_by_id))
}

fn release_doc_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(release_doc::create))
        .route("/{id}", get(release_doc::get))
}