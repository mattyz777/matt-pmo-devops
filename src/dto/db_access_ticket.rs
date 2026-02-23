use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbAccessTicketDto {
    pub title: String,
    pub items: Vec<String>,
}
