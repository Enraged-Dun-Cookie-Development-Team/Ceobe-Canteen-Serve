
use chrono::Local;
use sea_orm::{ entity::prelude::*, Set };


#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fetcher_datasource_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
}
