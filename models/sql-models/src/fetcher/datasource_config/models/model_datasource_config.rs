use sea_orm::{ entity::prelude::*, Set };
use sub_model::SubModel;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_datasource_config")]
#[sub_model(
    all(name = "BackendDatasource", extra(derive(sea_orm::FromQueryResult))),
    none(name = "FrontendDatasource", extra(derive(sea_orm::FromQueryResult))),
    none(name = "DataSourceForFetcherConfig", extra(derive(sea_orm::FromQueryResult))),
)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sub_model(want("DataSourceForFetcherConfig"))]
    pub id: i32,
    pub platform: String, 
    pub datasource: String,
    #[sub_model(want("FrontendDatasource"))]
    #[sub_model(want("DataSourceForFetcherConfig"))]
    pub nickname: String,
    #[sub_model(want("FrontendDatasource"))]
    pub avatar: String,
    #[sub_model(want("DataSourceForFetcherConfig"))]
    pub config: String,
    #[sub_model(ignore("BackendDatasource"), want("FrontendDatasource"))]
    pub unique_id: Uuid
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
}