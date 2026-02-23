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
    pub fn into_create_model(self, release_note_id: i32, operator_id: i32) -> crate::entity::feature::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::feature::ActiveModel {
            id: NotSet,
            release_note_id: Set(release_note_id),
            jira_id: Set(self.jira_id),
            link: Set(self.link),
            description: Set(self.description),
            is_delete: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            creator: Set(operator_id),
            updator: NotSet,
        }
    }

    pub fn into_update_model(self, operator_id: i32) -> crate::entity::feature::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::feature::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_note_id: NotSet,
            jira_id: Set(self.jira_id),
            link: Set(self.link),
            description: Set(self.description),
            is_delete: NotSet,
            created_at: NotSet,
            updated_at: Set(now),
            creator: NotSet,
            updator: Set(operator_id),
        }
    }
}
