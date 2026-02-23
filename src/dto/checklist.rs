use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistDto {
    pub title: String,
    pub items: Vec<String>,
}
