use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ReleaseEnvironment {
    #[sea_orm(string_value = "uat")]
    Uat,
    #[sea_orm(string_value = "prod")]
    Prod,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ReleaseType {
    #[sea_orm(string_value = "sprint")]
    Sprint,
    #[sea_orm(string_value = "hotfix")]
    Hotfix,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ProjectType {
    #[sea_orm(string_value = "onchain")]
    Onchain,
    #[sea_orm(string_value = "offchain")]
    Offchain,
    #[sea_orm(string_value = "frontend")]
    Frontend,
    #[sea_orm(string_value = "backend")]
    Backend,
    #[sea_orm(string_value = "risk")]
    Risk,
    #[sea_orm(string_value = "settlement")]
    Settlement,
}


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "release_docs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub version: String,

    pub env: ReleaseEnvironment,

    pub kind: ReleaseType,

    pub project_type: ProjectType,

    pub is_ready: bool,

    // JSON 字段存储子数据
    pub release_plans: Json,
    pub release_notes: Json,
    pub checklists: Json,
    pub db_access_tickets: Json,
    pub sql_review_tickets: Json,

    // 通用字段
    pub is_delete: bool,
    pub created_at: DateTime<Utc>, 
    pub updated_at: Option<DateTime<Utc>>,
    pub creator: i32,
    pub updator: Option<i32>,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
