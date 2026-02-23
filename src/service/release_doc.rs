use sea_orm::DatabaseConnection;
use crate::dto::{ReleaseDocDto, ReleaseNoteDto};
use crate::converter::{
    release_plan_entities_to_dtos, feature_entities_to_dtos, secure_report_entities_to_dtos,
    checklist_entities_to_dtos, db_access_ticket_entities_to_dtos, sql_review_ticket_entities_to_dtos,
    release_note_to_dto_with_relations, release_doc_to_dto_with_relations,
};
use crate::dao::{
    release_doc_dao, release_plan_dao, release_note_dao,
    checklist_dao, feature_dao, secure_report_dao,
    db_access_ticket_dao, sql_review_ticket_dao,
};

/// 获取 ReleaseNotes 及其关联的 Features 和 SecureReports
async fn fetch_release_notes_with_relations(
    db: &DatabaseConnection,
    release_doc_id: i32,
) -> Result<Vec<ReleaseNoteDto>, sea_orm::DbErr> {
    let release_note_entities = release_note_dao::get_by_release_doc_id(db, release_doc_id).await?;
    let mut release_notes = Vec::new();

    for note_entity in release_note_entities {
        let feature_entities = feature_dao::get_by_release_note_id(db, note_entity.id).await?;
        let features = feature_entities_to_dtos(feature_entities);

        let secure_report_entities = secure_report_dao::get_by_release_note_id(db, note_entity.id).await?;
        let secure_reports = secure_report_entities_to_dtos(secure_report_entities);

        release_notes.push(release_note_to_dto_with_relations(note_entity, features, secure_reports));
    }

    Ok(release_notes)
}

/// 根据 release_doc id 组装完整的 ReleaseDocDto 对象
///
/// # 参数
/// * `db` - 数据库连接
/// * `id` - 发布文档 ID
///
/// # 返回
/// * `Ok(Some(ReleaseDocDto))` - 找到并成功组装
/// * `Ok(None)` - 未找到该发布文档
/// * `Err(DbErr)` - 数据库错误
pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<ReleaseDocDto>, sea_orm::DbErr> {
    // 1. 获取 ReleaseDoc 主记录
    let release_doc_entity = match release_doc_dao::get_by_id(db, id).await? {
        Some(entity) => entity,
        None => return Ok(None),
    };

    // 2. 获取关联的 ReleasePlans
    let release_plan_entities = release_plan_dao::get_by_release_doc_id(db, id).await?;
    let release_plans = release_plan_entities_to_dtos(release_plan_entities);

    // 3. 获取关联的 ReleaseNotes 及其子关联
    let release_notes = fetch_release_notes_with_relations(db, id).await?;

    // 4. 获取关联的 Checklists
    let checklist_entities = checklist_dao::get_by_release_doc_id(db, id).await?;
    let checklists = checklist_entities_to_dtos(checklist_entities);

    // 5. 获取关联的 DbAccessTickets
    let db_access_ticket_entities = db_access_ticket_dao::get_by_release_doc_id(db, id).await?;
    let db_access_tickets = db_access_ticket_entities_to_dtos(db_access_ticket_entities);

    // 6. 获取关联的 SqlReviewTickets
    let sql_review_ticket_entities = sql_review_ticket_dao::get_by_release_doc_id(db, id).await?;
    let sql_review_tickets = sql_review_ticket_entities_to_dtos(sql_review_ticket_entities);

    // 7. 组装完整的 ReleaseDocDto
    let release_doc_dto = release_doc_to_dto_with_relations(
        release_doc_entity,
        release_plans,
        release_notes,
        checklists,
        db_access_tickets,
        sql_review_tickets,
    );

    Ok(Some(release_doc_dto))
}
