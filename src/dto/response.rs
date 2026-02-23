use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    pub fn success_without_data() -> ApiResponse<()> {
        ApiResponse {
            code: 0,
            data: None,
            message: "success".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            code: 500,
            data: None,
            message,
        }
    }
}
