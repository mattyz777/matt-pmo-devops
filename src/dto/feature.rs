use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDto {
    pub jira_id: String,
    pub link: String,
    pub description: String,
}
