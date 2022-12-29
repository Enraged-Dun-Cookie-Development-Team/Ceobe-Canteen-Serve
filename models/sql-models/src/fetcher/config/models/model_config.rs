
use chrono::Local;
use sea_orm::{ entity::prelude::*, Set };

use crate::get_zero_data_time;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fetcher_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub live_number: i8,
    pub fetcher_count: i8,
    pub group_name: String,
    pub platform: String,
    pub datasource_id: i32,
    pub interval: Option<i32>,
    pub interval_by_time_range: Option<String>,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
}
