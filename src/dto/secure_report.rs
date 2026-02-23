use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureReportDto {
    pub id: Option<i32>,
    pub link: String,
    pub note: String,
}

impl From<crate::entity::secure_report::Model> for SecureReportDto {
    fn from(entity: crate::entity::secure_report::Model) -> Self {
        Self {
            id: Some(entity.id),
            link: entity.link,
            note: entity.note,
        }
    }
}

impl SecureReportDto {
    pub fn into_create_model(self, release_note_id: i32, operator_id: i32) -> crate::entity::secure_report::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::secure_report::ActiveModel {
            id: NotSet,
            release_note_id: Set(release_note_id),
            link: Set(self.link),
            note: Set(self.note),
            is_delete: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            creator: Set(operator_id),
            updator: NotSet,
        }
    }

    pub fn into_update_model(self, operator_id: i32) -> crate::entity::secure_report::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::secure_report::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_note_id: NotSet,
            link: Set(self.link),
            note: Set(self.note),
            is_delete: NotSet,
            created_at: NotSet,
            updated_at: Set(now),
            creator: NotSet,
            updator: Set(operator_id),
        }
    }
}
