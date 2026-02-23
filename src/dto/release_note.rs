use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNoteDto {
    pub id: Option<i32>,
    pub job_name: String,
    pub git_tag: String,
    pub features: Vec<FeatureDto>,
    pub secure_reports: Vec<SecureReportDto>,
}

impl From<crate::entity::release_note::Model> for ReleaseNoteDto {
    fn from(entity: crate::entity::release_note::Model) -> Self {
        Self {
            id: Some(entity.id),
            job_name: entity.job_name,
            git_tag: entity.git_tag,
            features: Vec::new(),
            secure_reports: Vec::new(),
        }
    }
}

impl ReleaseNoteDto {
    pub fn into_create_model(self, release_doc_id: i32, operator_id: i32) -> crate::entity::release_note::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::release_note::ActiveModel {
            id: NotSet,
            release_doc_id: Set(release_doc_id),
            job_name: Set(self.job_name),
            git_tag: Set(self.git_tag),
            is_delete: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            creator: Set(operator_id),
            updator: NotSet,
        }
    }

    pub fn into_update_model(self, operator_id: i32) -> crate::entity::release_note::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::release_note::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_doc_id: NotSet,
            job_name: Set(self.job_name),
            git_tag: Set(self.git_tag),
            is_delete: NotSet,
            created_at: NotSet,
            updated_at: Set(now),
            creator: NotSet,
            updator: Set(operator_id),
        }
    }
}

use super::{FeatureDto, SecureReportDto};
