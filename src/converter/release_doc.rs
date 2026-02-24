use sea_orm::ActiveValue::{NotSet, Set};
use serde_json::Value as Json;
use crate::dto::{ReleaseDocRequestDto, ReleaseDocResponseDto, ReleaseEnvironment, ReleaseType, ProjectType};
use crate::entity::release_doc;


impl ReleaseDocRequestDto {
    pub fn into_create_model(self, operator_id: i32) -> release_doc::ActiveModel {
        let now = chrono::Utc::now();

        release_doc::ActiveModel {
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
            is_deleted: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            created_by: Set(operator_id),
            updated_by: NotSet,
        }
    }

    pub fn into_update_model(self, record_id: i32, operator_id: i32) -> release_doc::ActiveModel {
        let now = chrono::Utc::now();

        release_doc::ActiveModel {
            id: Set(record_id),
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
            is_deleted: NotSet,
            created_at: NotSet,
            updated_at: Set(Some(now)),
            created_by: NotSet,
            updated_by: Set(Some(operator_id)),
        }
    }
}


impl From<release_doc::Model> for ReleaseDocResponseDto {
    fn from(entity: release_doc::Model) -> Self {
        Self {
            id: entity.id,
            version: entity.version,
            env: release_env_entity_to_dto(entity.env),
            kind: release_type_entity_to_dto(entity.kind),
            project_type: project_type_entity_to_dto(entity.project_type),
            release_plans: serde_json::from_value(entity.release_plans).unwrap_or_default(),
            release_notes: serde_json::from_value(entity.release_notes).unwrap_or_default(),
            checklists: serde_json::from_value(entity.checklists).unwrap_or_default(),
            db_access_tickets: serde_json::from_value(entity.db_access_tickets).unwrap_or_default(),
            sql_review_tickets: serde_json::from_value(entity.sql_review_tickets).unwrap_or_default(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}


pub fn release_env_entity_to_dto(
    entity_env: release_doc::ReleaseEnvironment,
) -> ReleaseEnvironment {
    match entity_env {
        release_doc::ReleaseEnvironment::Uat => ReleaseEnvironment::Uat,
        release_doc::ReleaseEnvironment::Prod => ReleaseEnvironment::Prod,
    }
}


pub fn release_env_dto_to_entity(dto_env: ReleaseEnvironment) -> release_doc::ReleaseEnvironment {
    match dto_env {
        ReleaseEnvironment::Uat => release_doc::ReleaseEnvironment::Uat,
        ReleaseEnvironment::Prod => release_doc::ReleaseEnvironment::Prod,
    }
}


pub fn release_type_entity_to_dto(
    entity_kind: release_doc::ReleaseType,
) -> ReleaseType {
    match entity_kind {
        release_doc::ReleaseType::Sprint => ReleaseType::Sprint,
        release_doc::ReleaseType::Hotfix => ReleaseType::Hotfix,
    }
}


pub fn release_type_dto_to_entity(dto_kind: ReleaseType) -> release_doc::ReleaseType {
    match dto_kind {
        ReleaseType::Sprint => release_doc::ReleaseType::Sprint,
        ReleaseType::Hotfix => release_doc::ReleaseType::Hotfix,
    }
}


pub fn project_type_entity_to_dto(
    entity_kind: release_doc::ProjectType,
) -> ProjectType {
    match entity_kind {
        release_doc::ProjectType::Onchain => ProjectType::Onchain,
        release_doc::ProjectType::Offchain => ProjectType::Offchain,
        release_doc::ProjectType::Frontend => ProjectType::Frontend,
        release_doc::ProjectType::Backend => ProjectType::Backend,
        release_doc::ProjectType::Risk => ProjectType::Risk,
        release_doc::ProjectType::Settlement => ProjectType::Settlement,
    }
}


pub fn project_type_dto_to_entity(dto_kind: ProjectType) -> release_doc::ProjectType {
    match dto_kind {
        ProjectType::Onchain => release_doc::ProjectType::Onchain,
        ProjectType::Offchain => release_doc::ProjectType::Offchain,
        ProjectType::Frontend => release_doc::ProjectType::Frontend,
        ProjectType::Backend => release_doc::ProjectType::Backend,
        ProjectType::Risk => release_doc::ProjectType::Risk,
        ProjectType::Settlement => release_doc::ProjectType::Settlement,
    }
}
