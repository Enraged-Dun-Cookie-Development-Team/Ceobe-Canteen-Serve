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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "countdown_type")]
pub enum CountdownType {    
    #[serde(rename = "activity")]
    #[sea_orm(string_value = "activity")]
    Activity,
    #[serde(rename = "banner")]
    #[sea_orm(string_value = "banner")]
    Banner,
    #[serde(rename = "live")]
    #[sea_orm(string_value = "live")]
    Live,
}