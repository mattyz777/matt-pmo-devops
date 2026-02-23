use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Onchain,
    Offchain,
    Frontend,
    Backend,
    Risk,
    Settlement,
}

/// 发布文档 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDocDto {
    pub id: Option<i32>,
    pub version: String,
    pub env: ReleaseEnvironment,
    pub kind: ReleaseType,
    pub project_type: ProjectType,
    pub release_plans: Vec<ReleasePlanDto>,
    pub release_notes: Vec<ReleaseNoteDto>,
    pub db_access_tickets: Vec<DbAccessTicketDto>,
    pub sql_review_tickets: Vec<SqlReviewTicketDto>,
    pub checklists: Vec<ChecklistDto>,
}

impl From<crate::entity::release_doc::Model> for ReleaseDocDto {
    fn from(entity: crate::entity::release_doc::Model) -> Self {
        Self {
            id: Some(entity.id),
            version: entity.version,
            env: release_env_entity_to_dto(entity.env),
            kind: release_type_entity_to_dto(entity.kind),
            project_type: project_type_entity_to_dto(entity.project_type),
            release_plans: serde_json::from_value(entity.release_plans).unwrap_or_default(),
            release_notes: serde_json::from_value(entity.release_notes).unwrap_or_default(),
            checklists: serde_json::from_value(entity.checklists).unwrap_or_default(),
            db_access_tickets: serde_json::from_value(entity.db_access_tickets).unwrap_or_default(),
            sql_review_tickets: serde_json::from_value(entity.sql_review_tickets).unwrap_or_default(),
        }
    }
}

impl ReleaseDocDto {
    pub fn into_create_model(self, operator_id: i32) -> crate::entity::release_doc::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::release_doc::ActiveModel {
            id: NotSet,
            version: Set(self.version),
            env: Set(release_env_dto_to_entity(self.env)),
            kind: Set(release_type_dto_to_entity(self.kind)),
            project_type: Set(project_type_dto_to_entity(self.project_type)),
            is_ready: Set(false),
            release_plans: Set(serde_json::to_value(self.release_plans).unwrap_or(Json::Null)),
            release_notes: Set(serde_json::to_value(self.release_notes).unwrap_or(Json::Null)),
            checklists: Set(serde_json::to_value(self.checklists).unwrap_or(Json::Null)),
            db_access_tickets: Set(serde_json::to_value(self.db_access_tickets).unwrap_or(Json::Null)),
            sql_review_tickets: Set(serde_json::to_value(self.sql_review_tickets).unwrap_or(Json::Null)),
            is_delete: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            creator: Set(operator_id),
            updator: NotSet,
        }
    }

    pub fn into_update_model(self, operator_id: i32) -> crate::entity::release_doc::ActiveModel {
        let now = chrono::Utc::now().naive_utc();

        crate::entity::release_doc::ActiveModel {
            id: self.id.map(Set).unwrap_or(NotSet),
            version: Set(self.version),
            env: Set(release_env_dto_to_entity(self.env)),
            kind: Set(release_type_dto_to_entity(self.kind)),
            project_type: Set(project_type_dto_to_entity(self.project_type)),
            is_ready: NotSet,
            release_plans: Set(serde_json::to_value(self.release_plans).unwrap_or(Json::Null)),
            release_notes: Set(serde_json::to_value(self.release_notes).unwrap_or(Json::Null)),
            checklists: Set(serde_json::to_value(self.checklists).unwrap_or(Json::Null)),
            db_access_tickets: Set(serde_json::to_value(self.db_access_tickets).unwrap_or(Json::Null)),
            sql_review_tickets: Set(serde_json::to_value(self.sql_review_tickets).unwrap_or(Json::Null)),
            is_delete: NotSet,
            created_at: NotSet,
            updated_at: Set(Some(now)),
            creator: NotSet,
            updator: Set(Some(operator_id)),
        }
    }
}

pub fn release_env_entity_to_dto(
    entity_env: crate::entity::release_doc::ReleaseEnvironment,
) -> ReleaseEnvironment {
    match entity_env {
        crate::entity::release_doc::ReleaseEnvironment::Uat => ReleaseEnvironment::Uat,
        crate::entity::release_doc::ReleaseEnvironment::Prod => ReleaseEnvironment::Prod,
    }
}

pub fn release_env_dto_to_entity(dto_env: ReleaseEnvironment) -> crate::entity::release_doc::ReleaseEnvironment {
    match dto_env {
        ReleaseEnvironment::Uat => crate::entity::release_doc::ReleaseEnvironment::Uat,
        ReleaseEnvironment::Prod => crate::entity::release_doc::ReleaseEnvironment::Prod,
    }
}

pub fn release_type_entity_to_dto(
    entity_kind: crate::entity::release_doc::ReleaseType,
) -> ReleaseType {
    match entity_kind {
        crate::entity::release_doc::ReleaseType::Sprint => ReleaseType::Sprint,
        crate::entity::release_doc::ReleaseType::Hotfix => ReleaseType::Hotfix,
    }
}

pub fn release_type_dto_to_entity(dto_kind: ReleaseType) -> crate::entity::release_doc::ReleaseType {
    match dto_kind {
        ReleaseType::Sprint => crate::entity::release_doc::ReleaseType::Sprint,
        ReleaseType::Hotfix => crate::entity::release_doc::ReleaseType::Hotfix,
    }
}

pub fn project_type_entity_to_dto(
    entity_kind: crate::entity::release_doc::ProjectType,
) -> ProjectType {
    match entity_kind {
        crate::entity::release_doc::ProjectType::Onchain => ProjectType::Onchain,
        crate::entity::release_doc::ProjectType::Offchain => ProjectType::Offchain,
        crate::entity::release_doc::ProjectType::Frontend => ProjectType::Frontend,
        crate::entity::release_doc::ProjectType::Backend => ProjectType::Backend,
        crate::entity::release_doc::ProjectType::Risk => ProjectType::Risk,
        crate::entity::release_doc::ProjectType::Settlement => ProjectType::Settlement,
    }
}

pub fn project_type_dto_to_entity(dto_kind: ProjectType) -> crate::entity::release_doc::ProjectType {
    match dto_kind {
        ProjectType::Onchain => crate::entity::release_doc::ProjectType::Onchain,
        ProjectType::Offchain => crate::entity::release_doc::ProjectType::Offchain,
        ProjectType::Frontend => crate::entity::release_doc::ProjectType::Frontend,
        ProjectType::Backend => crate::entity::release_doc::ProjectType::Backend,
        ProjectType::Risk => crate::entity::release_doc::ProjectType::Risk,
        ProjectType::Settlement => crate::entity::release_doc::ProjectType::Settlement,
    }
}
