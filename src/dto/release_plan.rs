use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleasePlanDto {
    pub id: Option<i32>,
    pub job_name: String,
    pub tag: String,
    pub git_url: String,
    pub rollback_tag: String,
}

impl From<crate::entity::release_plan::Model> for ReleasePlanDto {
    fn from(entity: crate::entity::release_plan::Model) -> Self {
        Self {
            id: Some(entity.id),
            job_name: entity.job_name,
            tag: entity.tag,
            git_url: entity.git_url,
            rollback_tag: entity.rollback_tag,
        }
    }
}

impl ReleasePlanDto {
    pub fn into_active_model(self, release_doc_id: i32, operator_id: i32) -> crate::entity::release_plan::ActiveModel {
        let now = chrono::Utc::now().naive_utc();
        let is_update = self.id.is_some();

        crate::entity::release_plan::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_doc_id: if is_update { NotSet } else { Set(release_doc_id) },
            job_name: Set(self.job_name),
            tag: Set(self.tag),
            git_url: Set(self.git_url),
            rollback_tag: Set(self.rollback_tag),
            is_delete: if is_update { NotSet } else { Set(false) },
            created_at: if is_update { NotSet } else { Set(now) },
            updated_at: Set(Some(now)),
            creator: if is_update { NotSet } else { Set(operator_id) },
            updator: Set(operator_id),
        }
    }
}
