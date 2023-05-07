use sea_orm::{entity::prelude::*, ActiveValue};
use sub_model::SubModel;

use crate::{
    fetcher::{
        datasource_config::checkers::DatasourceUnique,
        platform_config::models::model_platform_config,
    },
    NaiveDateTime, SoftDelete,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_datasource_config")]
#[sub_model(
    all(
        name = "BackendDatasource",
        extra(derive(sea_orm::FromQueryResult, serde::Serialize, Debug))
    ),
    none(
        name = "FrontendDatasource",
        extra(derive(sea_orm::FromQueryResult, serde::Serialize, Debug))
    ),
    none(
        name = "DatasourceBasicInfo",
        extra(derive(sea_orm::FromQueryResult, serde::Serialize, Debug))
    ),
    none(name = "DatasourceUuid", extra(derive(sea_orm::FromQueryResult))),
    none(
        name = "SingleDatasourceInfo",
        extra(derive(sea_orm::FromQueryResult))
    ),
    none(
        name = "DataSourceForFetcherConfig",
        extra(derive(sea_orm::FromQueryResult))
    ),
    none(
        name = "DatasourcePlatform",
        extra(
            derive(sea_orm::FromQueryResult),
            doc = "取得数据源所属的平台"
        )
    ),
    none(
        name = "DatasourceId",
        extra(derive(sea_orm::FromQueryResult), doc = "取得数据源对应id")
    )
)]
pub struct Model {
    /// 平台type
    #[sea_orm(primary_key)]
    #[sub_model(want("DataSourceForFetcherConfig"), want("DatasourceId"), want("DatasourceBasicInfo"))]
    pub id: i32,
    #[sub_model(want("DatasourcePlatform"))]
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
        want("FrontendDatasource"),
        want("DatasourceBasicInfo")
    )]
    pub nickname: String,
    /// 数据源头像
    #[sub_model(want("FrontendDatasource"),
    want("DatasourceBasicInfo"))]
    pub avatar: String,
    /// 数据源配置
    #[sub_model(want("DataSourceForFetcherConfig"))]
    pub config: String,
    /// 数据源uuid，给用户端使用
    #[sub_model(
        ignore("BackendDatasource"),
        want("FrontendDatasource"),
        want("DatasourceUuid")
    )]
    pub unique_id: Uuid,
    /// 数据库使用的Unique Key
    #[sub_model(ignore("BackendDatasource"))]
    pub db_unique_key: DatasourceUnique,
    /// field for soft delete
    #[sub_model(ignore("BackendDatasource"))]
    pub(in crate::fetcher::datasource_config) delete_at: DateTime,
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

impl SoftDelete for ActiveModel {
    fn get_mut(&mut self) -> &mut ActiveValue<NaiveDateTime> {
        &mut self.delete_at
    }
}
