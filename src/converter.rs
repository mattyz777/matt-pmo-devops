//! Entity 与 BO 之间的转换模块
//!
//! 本模块集中管理数据库实体 (entity) 与业务对象 (bo) 之间的转换逻辑

use crate::bo::{
    ReleaseDocBo, ReleasePlanBo, ReleaseNoteBo, ChecklistBo,
    DbAccessTicketBo, SqlReviewTicketBo, FeatureBo, SecureReportBo,
    ReleaseEnvironment, ReleaseType,
};

// ============================================================================
// ReleaseDoc 相关转换
// ============================================================================
pub fn release_doc_to_bo_with_relations(
    entity: crate::entity::release_doc::Model,
    release_plans: Vec<ReleasePlanBo>,
    release_notes: Vec<ReleaseNoteBo>,
    checklists: Vec<ChecklistBo>,
    db_access_tickets: Vec<DbAccessTicketBo>,
    sql_review_tickets: Vec<SqlReviewTicketBo>,
) -> ReleaseDocBo {
    ReleaseDocBo {
        id: entity.id,
        version: entity.version,
        env: release_env_entity_to_bo(entity.env),
        kind: release_type_entity_to_bo(entity.kind),
        release_plans,
        release_notes,
        checklists,
        db_access_tickets,
        sql_review_tickets,
    }
}

pub fn release_env_entity_to_bo(
    entity_env: crate::entity::release_doc::ReleaseEnvironment,
) -> ReleaseEnvironment {
    match entity_env {
        crate::entity::release_doc::ReleaseEnvironment::Uat => ReleaseEnvironment::Uat,
        crate::entity::release_doc::ReleaseEnvironment::Prod => ReleaseEnvironment::Prod,
    }
}

pub fn release_env_bo_to_entity(bo_env: ReleaseEnvironment) -> crate::entity::release_doc::ReleaseEnvironment {
    match bo_env {
        ReleaseEnvironment::Uat => crate::entity::release_doc::ReleaseEnvironment::Uat,
        ReleaseEnvironment::Prod => crate::entity::release_doc::ReleaseEnvironment::Prod,
    }
}

pub fn release_type_entity_to_bo(
    entity_kind: crate::entity::release_doc::ReleaseType,
) -> ReleaseType {
    match entity_kind {
        crate::entity::release_doc::ReleaseType::Sprint => ReleaseType::Sprint,
        crate::entity::release_doc::ReleaseType::Hotfix => ReleaseType::Hotfix,
    }
}

pub fn release_type_bo_to_entity(bo_kind: ReleaseType) -> crate::entity::release_doc::ReleaseType {
    match bo_kind {
        ReleaseType::Sprint => crate::entity::release_doc::ReleaseType::Sprint,
        ReleaseType::Hotfix => crate::entity::release_doc::ReleaseType::Hotfix,
    }
}

// ============================================================================
// ReleasePlan 相关转换
// ============================================================================
impl From<crate::entity::release_plan::Model> for ReleasePlanBo {
    fn from(entity: crate::entity::release_plan::Model) -> Self {
        Self {
            id: entity.id,
            job_name: entity.job_name,
            tag: entity.tag,
            git_url: entity.git_url,
            rollback_tag: entity.rollback_tag,
        }
    }
}

pub fn release_plan_entities_to_bos(
    entities: Vec<crate::entity::release_plan::Model>,
) -> Vec<ReleasePlanBo> {
    entities.into_iter().map(|e| e.into()).collect()
}

// ============================================================================
// ReleaseNote 相关转换
// ============================================================================
pub fn release_note_to_bo_with_relations(
    entity: crate::entity::release_note::Model,
    features: Vec<FeatureBo>,
    secure_reports: Vec<SecureReportBo>,
) -> ReleaseNoteBo {
    ReleaseNoteBo {
        id: entity.id,
        job_name: entity.job_name,
        git_tag: entity.git_tag,
        features,
        secure_reports,
    }
}

// ============================================================================
// Feature 相关转换
// ============================================================================
impl From<crate::entity::feature::Model> for FeatureBo {
    fn from(entity: crate::entity::feature::Model) -> Self {
        Self {
            id: entity.id,
            jira_id: entity.jira_id,
            link: entity.link,
            description: entity.description,
        }
    }
}

pub fn feature_entities_to_bos(
    entities: Vec<crate::entity::feature::Model>,
) -> Vec<FeatureBo> {
    entities.into_iter().map(|e| e.into()).collect()
}

// ============================================================================
// SecureReport 相关转换
// ============================================================================
impl From<crate::entity::secure_report::Model> for SecureReportBo {
    fn from(entity: crate::entity::secure_report::Model) -> Self {
        Self {
            id: entity.id,
            link: entity.link,
            note: entity.note,
        }
    }
}

pub fn secure_report_entities_to_bos(
    entities: Vec<crate::entity::secure_report::Model>,
) -> Vec<SecureReportBo> {
    entities.into_iter().map(|e| e.into()).collect()
}

// ============================================================================
// Checklist 相关转换
// ============================================================================
impl From<crate::entity::checklist::Model> for ChecklistBo {
    fn from(entity: crate::entity::checklist::Model) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            items: entity.items,
        }
    }
}

pub fn checklist_entities_to_bos(
    entities: Vec<crate::entity::checklist::Model>,
) -> Vec<ChecklistBo> {
    entities.into_iter().map(|e| e.into()).collect()
}

// ============================================================================
// DbAccessTicket 相关转换
// ============================================================================
impl From<crate::entity::db_access_ticket::Model> for DbAccessTicketBo {
    fn from(entity: crate::entity::db_access_ticket::Model) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            items: entity.items,
        }
    }
}

pub fn db_access_ticket_entities_to_bos(
    entities: Vec<crate::entity::db_access_ticket::Model>,
) -> Vec<DbAccessTicketBo> {
    entities.into_iter().map(|e| e.into()).collect()
}

// ============================================================================
// SqlReviewTicket 相关转换
// ============================================================================
impl From<crate::entity::sql_review_ticket::Model> for SqlReviewTicketBo {
    fn from(entity: crate::entity::sql_review_ticket::Model) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            items: entity.items,
        }
    }
}

pub fn sql_review_ticket_entities_to_bos(
    entities: Vec<crate::entity::sql_review_ticket::Model>,
) -> Vec<SqlReviewTicketBo> {
    entities.into_iter().map(|e| e.into()).collect()
}
