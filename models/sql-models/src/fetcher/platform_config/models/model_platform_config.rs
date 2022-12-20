use sea_orm::{ entity::prelude::* };

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fetcher_platform_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub type_id: String,
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
