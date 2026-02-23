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
    pub fn into_active_model(self, release_note_id: i32, operator_id: i32) -> crate::entity::secure_report::ActiveModel {
        let now = chrono::Utc::now().naive_utc();
        let is_update = self.id.is_some();

        crate::entity::secure_report::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_note_id: if is_update { NotSet } else { Set(release_note_id) },
            link: Set(self.link),
            note: Set(self.note),
            is_delete: if is_update { NotSet } else { Set(false) },
            created_at: if is_update { NotSet } else { Set(now) },
            updated_at: Set(Some(now)),
            creator: if is_update { NotSet } else { Set(operator_id) },
            updator: Set(operator_id),
        }
    }
}
