use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDto {
    pub id: Option<i32>,
    pub jira_id: String,
    pub link: String,
    pub description: String,
}

impl From<crate::entity::feature::Model> for FeatureDto {
    fn from(entity: crate::entity::feature::Model) -> Self {
        Self {
            id: Some(entity.id),
            jira_id: entity.jira_id,
            link: entity.link,
            description: entity.description,
        }
    }
}

impl FeatureDto {
    pub fn into_active_model(self, release_note_id: i32, operator_id: i32) -> crate::entity::feature::ActiveModel {
        let now = chrono::Utc::now().naive_utc();
        let is_update = self.id.is_some();

        crate::entity::feature::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_note_id: if is_update { NotSet } else { Set(release_note_id) },
            jira_id: Set(self.jira_id),
            link: Set(self.link),
            description: Set(self.description),
            is_delete: if is_update { NotSet } else { Set(false) },
            created_at: if is_update { NotSet } else { Set(now) },
            updated_at: Set(Some(now)),
            creator: if is_update { NotSet } else { Set(operator_id) },
            updator: Set(operator_id),
        }
    }
}
