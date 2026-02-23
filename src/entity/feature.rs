use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "features")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub release_note_id: i32,

    pub jira_id: String,

    pub link: String,

    pub description: String,

    // 通用字段
    pub is_delete: bool,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub creator: i32,
    pub updator: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
