//! DTO 与 Entity 之间的批量转换辅助模块
//!
//! 大多数简单转换使用 DTO 的 From trait 实现
//! 本模块只保留批量转换和带关联数据的组装逻辑

use crate::dto::{ReleaseDocDto, ReleaseNoteDto, FeatureDto, SecureReportDto};

// ============================================================================
// ReleaseDoc 组装（带关联数据）
// ============================================================================

/// 将 ReleaseDoc entity 及其所有关联数据组装成完整的 ReleaseDocDto
pub fn release_doc_to_dto_with_relations(
    entity: crate::entity::release_doc::Model,
    release_plans: Vec<crate::dto::ReleasePlanDto>,
    release_notes: Vec<crate::dto::ReleaseNoteDto>,
    checklists: Vec<crate::dto::ChecklistDto>,
    db_access_tickets: Vec<crate::dto::DbAccessTicketDto>,
    sql_review_tickets: Vec<crate::dto::SqlReviewTicketDto>,
) -> ReleaseDocDto {
    let mut dto: ReleaseDocDto = entity.into();
    dto.release_plans = release_plans;
    dto.release_notes = release_notes;
    dto.checklists = checklists;
    dto.db_access_tickets = db_access_tickets;
    dto.sql_review_tickets = sql_review_tickets;
    dto
}

// ============================================================================
// ReleaseNote 组装（带关联数据）
// ============================================================================

/// 将 ReleaseNote entity 及其关联数据组装成完整的 ReleaseNoteDto
pub fn release_note_to_dto_with_relations(
    entity: crate::entity::release_note::Model,
    features: Vec<FeatureDto>,
    secure_reports: Vec<SecureReportDto>,
) -> ReleaseNoteDto {
    let mut dto: ReleaseNoteDto = entity.into();
    dto.features = features;
    dto.secure_reports = secure_reports;
    dto
}

// ============================================================================
// 批量转换辅助函数
// ============================================================================

/// 批量转换 ReleasePlan entity 到 DTO
pub fn release_plan_entities_to_dtos(
    entities: Vec<crate::entity::release_plan::Model>,
) -> Vec<crate::dto::ReleasePlanDto> {
    entities.into_iter().map(|e| e.into()).collect()
}

/// 批量转换 Feature entity 到 DTO
pub fn feature_entities_to_dtos(
    entities: Vec<crate::entity::feature::Model>,
) -> Vec<FeatureDto> {
    entities.into_iter().map(|e| e.into()).collect()
}

/// 批量转换 SecureReport entity 到 DTO
pub fn secure_report_entities_to_dtos(
    entities: Vec<crate::entity::secure_report::Model>,
) -> Vec<SecureReportDto> {
    entities.into_iter().map(|e| e.into()).collect()
}

/// 批量转换 Checklist entity 到 DTO
pub fn checklist_entities_to_dtos(
    entities: Vec<crate::entity::checklist::Model>,
) -> Vec<crate::dto::ChecklistDto> {
    entities.into_iter().map(|e| e.into()).collect()
}

/// 批量转换 DbAccessTicket entity 到 DTO
pub fn db_access_ticket_entities_to_dtos(
    entities: Vec<crate::entity::db_access_ticket::Model>,
) -> Vec<crate::dto::DbAccessTicketDto> {
    entities.into_iter().map(|e| e.into()).collect()
}

/// 批量转换 SqlReviewTicket entity 到 DTO
pub fn sql_review_ticket_entities_to_dtos(
    entities: Vec<crate::entity::sql_review_ticket::Model>,
) -> Vec<crate::dto::SqlReviewTicketDto> {
    entities.into_iter().map(|e| e.into()).collect()
}
