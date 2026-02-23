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
    pub fn into_create_model(self, release_doc_id: i32, operator_id: i32) -> crate::entity::db_access_ticket::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::db_access_ticket::ActiveModel {
            id: NotSet,
            release_doc_id: Set(release_doc_id),
            title: Set(self.title),
            items: Set(self.items),
            is_delete: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            creator: Set(operator_id),
            updator: NotSet,
        }
    }

    pub fn into_update_model(self, operator_id: i32) -> crate::entity::db_access_ticket::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::db_access_ticket::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            release_doc_id: NotSet,
            title: Set(self.title),
            items: Set(self.items),
            is_delete: NotSet,
            created_at: NotSet,
            updated_at: Set(now),
            creator: NotSet,
            updator: Set(operator_id),
        }
    }
}
