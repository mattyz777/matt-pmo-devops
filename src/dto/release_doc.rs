use serde::{Deserialize, Serialize};
use super::{ReleasePlanDto, ReleaseNoteDto, ChecklistDto};
use chrono::{DateTime, Utc};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Onchain,
    Offchain,
    Frontend,
    Backend,
    Risk,
    Settlement,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDocRequestDto {
    pub version: String,
    pub env: ReleaseEnvironment,
    pub kind: ReleaseType,
    pub project_type: ProjectType,
    pub release_plans: Vec<ReleasePlanDto>,
    pub release_notes: Vec<ReleaseNoteDto>,
    pub db_access_tickets: Vec<String>,
    pub sql_review_tickets: Vec<String>,
    pub checklists: Vec<ChecklistDto>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDocResponseDto {
    pub id: i32,
    pub version: String,
    pub env: ReleaseEnvironment,
    pub kind: ReleaseType,
    pub project_type: ProjectType,
    pub release_plans: Vec<ReleasePlanDto>,
    pub release_notes: Vec<ReleaseNoteDto>,
    pub db_access_tickets: Vec<String>,
    pub sql_review_tickets: Vec<String>,
    pub checklists: Vec<ChecklistDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
