use sea_orm::entity::prelude::*;
use sub_model::SubModel;

use crate::fetcher::platform_config::models::model_platform_config;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_datasource_config")]
#[sub_model(
    all(
        name = "BackendDatasource",
        extra(derive(sea_orm::FromQueryResult, serde::Serialize, Debug))
    ),
    none(
        name = "FrontendDatasource",
        extra(derive(sea_orm::FromQueryResult))
    ),
    none(
        name = "SingleDatasourceInfo",
        extra(derive(sea_orm::FromQueryResult))
    ),
    none(
        name = "DataSourceForFetcherConfig",
        extra(derive(sea_orm::FromQueryResult))
    )
)]
pub struct Model {
    /// 平台type
    #[sea_orm(primary_key)]
    #[sub_model(want("DataSourceForFetcherConfig"))]
    pub id: i32,
    pub platform: String,
    /// 数据源type
    #[sub_model(
        want("SingleDatasourceInfo"),
        want("DataSourceForFetcherConfig")
    )]
    pub datasource: String,
    /// 数据源名字
    #[sub_model(
        want("DataSourceForFetcherConfig"),
        want("FrontendDatasource")
    )]
    pub nickname: String,
    /// 数据源头像
    #[sub_model(want("FrontendDatasource"))]
    pub avatar: String,
    /// 数据源配置
    #[sub_model(want("DataSourceForFetcherConfig"))]
    pub config: String,
    /// 数据源uuid，给用户端使用
    #[sub_model(ignore("BackendDatasource"), want("FrontendDatasource"))]
    pub unique_id: Uuid,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {
    PlatForm,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::PlatForm => {
                Entity::belongs_to(model_platform_config::Entity)
                    .from(Column::Platform)
                    .to(model_platform_config::Column::TypeId)
                    .into()
            }
        }
    }
}

impl Related<model_platform_config::Entity> for Entity {
    fn to() -> RelationDef { Relation::PlatForm.def() }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {}
