use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleasePlanDto {
    pub job_name: String,
    pub tag: String,
    pub git_url: String,
    pub rollback_tag: String,
}
