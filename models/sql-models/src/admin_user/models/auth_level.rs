use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "auth")]
pub enum AuthLevel {
    #[serde(rename = "chef")]
    #[sea_orm(string_value = "chef")]
    Chef,
    #[serde(rename = "cooker")]
    #[sea_orm(string_value = "cooker")]
    Cooker,
    #[serde(rename = "architect")]
    #[sea_orm(string_value = "architect")]
    Architect,
}
