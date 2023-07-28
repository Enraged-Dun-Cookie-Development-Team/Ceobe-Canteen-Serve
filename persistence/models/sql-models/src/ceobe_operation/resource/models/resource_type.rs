use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "resource_type")]
pub enum ResourceType {
    #[sea_orm(string_value = "resource_all_available")]
    ResourceAllAvailable,
    #[sea_orm(string_value = "countdown")]
    Countdown,
}
