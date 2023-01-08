use sea_orm::entity::prelude::*;
use sub_model::SubModel;

use crate::fetcher::datasource_config::models::model_datasource_config;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_platform_config")]
#[sub_model(
    all(
        name = "PlatformWithHasDatasource",
        extra_field(has_datasource(ty = "bool", from = "Default::default")),
        extra(derive(serde::Serialize, Debug))
    ),
    none(name = "PlatformType", extra(derive(sea_orm::FromQueryResult))),
    none(
        name = "PlatformBasicInfo",
        extra(derive(sea_orm::FromQueryResult, serde::Serialize))
    )
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 平台type
    #[sub_model(want("PlatformType"), want("PlatformBasicInfo"))]
    pub type_id: String,
    /// 平台名字
    #[sub_model(want("PlatformBasicInfo"))]
    pub platform_name: String,
    /// 最小蹲饼间隔
    #[sub_model(want("PlatformBasicInfo"))]
    pub min_request_interval: u64,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {
    DataSource
}



impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::DataSource => Entity::has_many(model_datasource_config::Entity).into(),
        }
    }
}

impl Related<model_datasource_config::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataSource.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

