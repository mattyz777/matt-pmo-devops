use serde::{Deserialize, Serialize};
use super::{FeatureDto, SecureReportDto};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNoteDto {
    pub job_name: String,
    pub git_tag: String,
    pub features: Vec<FeatureDto>,
    pub secure_reports: Vec<SecureReportDto>,
}
