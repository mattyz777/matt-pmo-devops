use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbAccessTicketDto {
    pub id: Option<i32>,
    pub title: String,
    pub items: Vec<String>,
}

impl From<crate::entity::db_access_ticket::Model> for DbAccessTicketDto {
    fn from(entity: crate::entity::db_access_ticket::Model) -> Self {
        Self {
            id: Some(entity.id),
            title: entity.title,
            items: entity.items,
        }
    }
}

impl DbAccessTicketDto {
    pub fn into_active_model(self, release_doc_id: i32, operator_id: i32) -> crate::entity::db_access_ticket::ActiveModel {
        let now = chrono::Utc::now().naive_utc();
        let is_update = self.id.is_some();

        crate::entity::db_access_ticket::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_doc_id: if is_update { NotSet } else { Set(release_doc_id) },
            title: Set(self.title),
            items: Set(self.items),
            is_delete: if is_update { NotSet } else { Set(false) },
            created_at: if is_update { NotSet } else { Set(now) },
            updated_at: Set(Some(now)),
            creator: if is_update { NotSet } else { Set(operator_id) },
            updator: Set(operator_id),
        }
    }
}
