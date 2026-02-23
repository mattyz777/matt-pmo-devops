use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlReviewTicketDto {
    pub title: String,
    pub items: Vec<String>,
}
