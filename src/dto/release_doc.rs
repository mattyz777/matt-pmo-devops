use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use super::{ReleasePlanDto, ReleaseNoteDto, ChecklistDto, DbAccessTicketDto, SqlReviewTicketDto};

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

/// 发布文档 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDocDto {
    pub id: Option<i32>,
    pub version: String,
    pub env: ReleaseEnvironment,
    pub kind: ReleaseType,
    pub release_plans: Vec<ReleasePlanDto>,
    pub release_notes: Vec<ReleaseNoteDto>,
    pub checklists: Vec<ChecklistDto>,
    pub db_access_tickets: Vec<DbAccessTicketDto>,
    pub sql_review_tickets: Vec<SqlReviewTicketDto>,
}

impl From<crate::entity::release_doc::Model> for ReleaseDocDto {
    fn from(entity: crate::entity::release_doc::Model) -> Self {
        Self {
            id: Some(entity.id),
            version: entity.version,
            env: release_env_entity_to_dto(entity.env),
            kind: release_type_entity_to_dto(entity.kind),
            release_plans: Vec::new(),
            release_notes: Vec::new(),
            checklists: Vec::new(),
            db_access_tickets: Vec::new(),
            sql_review_tickets: Vec::new(),
        }
    }
}

impl ReleaseDocDto {
    pub fn into_active_model(self, operator_id: i32) -> crate::entity::release_doc::ActiveModel {
        let now = chrono::Utc::now().naive_utc();
        let is_update = self.id.is_some();

        crate::entity::release_doc::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            version: Set(self.version),
            env: Set(release_env_dto_to_entity(self.env)),
            kind: Set(release_type_dto_to_entity(self.kind)),
            is_delete: if is_update { NotSet } else { Set(false) },
            created_at: if is_update { NotSet } else { Set(now) },
            updated_at: Set(Some(now)),
            creator: if is_update { NotSet } else { Set(operator_id) },
            updator: Set(operator_id),
        }
    }
}

fn release_env_entity_to_dto(
    entity_env: crate::entity::release_doc::ReleaseEnvironment,
) -> ReleaseEnvironment {
    match entity_env {
        crate::entity::release_doc::ReleaseEnvironment::Uat => ReleaseEnvironment::Uat,
        crate::entity::release_doc::ReleaseEnvironment::Prod => ReleaseEnvironment::Prod,
    }
}

fn release_env_dto_to_entity(dto_env: ReleaseEnvironment) -> crate::entity::release_doc::ReleaseEnvironment {
    match dto_env {
        ReleaseEnvironment::Uat => crate::entity::release_doc::ReleaseEnvironment::Uat,
        ReleaseEnvironment::Prod => crate::entity::release_doc::ReleaseEnvironment::Prod,
    }
}

fn release_type_entity_to_dto(
    entity_kind: crate::entity::release_doc::ReleaseType,
) -> ReleaseType {
    match entity_kind {
        crate::entity::release_doc::ReleaseType::Sprint => ReleaseType::Sprint,
        crate::entity::release_doc::ReleaseType::Hotfix => ReleaseType::Hotfix,
    }
}

fn release_type_dto_to_entity(dto_kind: ReleaseType) -> crate::entity::release_doc::ReleaseType {
    match dto_kind {
        ReleaseType::Sprint => crate::entity::release_doc::ReleaseType::Sprint,
        ReleaseType::Hotfix => crate::entity::release_doc::ReleaseType::Hotfix,
    }
}

