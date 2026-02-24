use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ReleaseEnvironment {
    #[sea_orm(num_value = 1)]
    Uat,
    #[sea_orm(num_value = 2)]
    Prod,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ReleaseType {
    #[sea_orm(num_value = 1)]
    Sprint,
    #[sea_orm(num_value = 2)]
    Hotfix,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ProjectType {
    #[sea_orm(num_value = 1)]
    Onchain,
    #[sea_orm(num_value = 2)]
    Offchain,
    #[sea_orm(num_value = 3)]
    Frontend,
    #[sea_orm(num_value = 4)]
    Backend,
    #[sea_orm(num_value = 5)]
    Risk,
    #[sea_orm(num_value = 6)]
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
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>, 
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: i32,
    pub updated_by: Option<i32>,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
