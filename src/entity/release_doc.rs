use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


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


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "release_docs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub version: String,

    pub env: ReleaseEnvironment,

    pub kind: ReleaseType,

    // 通用字段
    pub is_delete: bool,
    pub cdate: DateTime,
    pub edate: DateTime,
    pub creator: i32,
    pub updator: i32,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
