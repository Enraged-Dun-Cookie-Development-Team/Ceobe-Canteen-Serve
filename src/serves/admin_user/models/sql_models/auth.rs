use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "auth")]
pub enum Auth {
    #[sea_orm(string_value = "chef")]
    Chef,
    #[sea_orm(string_value = "cooker")]
    Cooker,
    #[sea_orm(string_value = "architect")]
    Architect,
}
