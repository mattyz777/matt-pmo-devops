use axum::Json;
use serde_json::{Value, json};

pub async fn get_foo() -> String {
    "get foo".to_string()
}

pub async fn post_foo() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
