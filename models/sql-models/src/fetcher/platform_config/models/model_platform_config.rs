use sea_orm::{ entity::prelude::* };
use sub_model::SubModel;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_platform_config")]
#[sub_model(
    all(
        name = "PlatformWithHasDatasource",
        extra_field(
            has_datasource(ty = "bool", from = "Default::default")
        ),
        extra(derive(serde::Serialize, Debug))   
    ),
    none(name = "PlatformType", extra(derive(sea_orm::FromQueryResult))),
    none(name = "PlatformBasicInfo", extra(derive(sea_orm::FromQueryResult, serde::Serialize)))
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sub_model(want("PlatformType"), want("PlatformBasicInfo"))]
    pub type_id: String,
    #[sub_model(want("PlatformBasicInfo"))]
    pub platform_name: String,
    pub min_request_interval: i32,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {}
