use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseEnvironment {
    Uat,
    Prod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseType {
    Sprint,
    Hotfix,
}

// ============================================================================
// Core Business Objects
// ============================================================================

/// 发布文档业务对象 - 聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDocBo {
    pub id: i32,
    pub version: String,
    pub env: ReleaseEnvironment,
    pub kind: ReleaseType,
    pub release_plans: Vec<ReleasePlanBo>,
    pub release_notes: Vec<ReleaseNoteBo>,
    pub checklists: Vec<ChecklistBo>,
    pub db_access_tickets: Vec<DbAccessTicketBo>,
    pub sql_review_tickets: Vec<SqlReviewTicketBo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleasePlanBo {
    pub id: i32,
    pub job_name: String,
    pub tag: String,
    pub git_url: String,
    pub rollback_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNoteBo {
    pub id: i32,
    pub job_name: String,
    pub git_tag: String,
    pub features: Vec<FeatureBo>,
    pub secure_reports: Vec<SecureReportBo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistBo {
    pub id: i32,
    pub title: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbAccessTicketBo {
    pub id: i32,
    pub title: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlReviewTicketBo {
    pub id: i32,
    pub title: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureBo {
    pub id: i32,
    pub jira_id: String,
    pub link: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureReportBo {
    pub id: i32,
    pub link: String,
    pub note: String,
}
