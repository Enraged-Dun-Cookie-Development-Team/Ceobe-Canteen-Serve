use sea_orm::{entity::prelude::*, Set, IntoActiveModel};
use sub_model::SubModel;
use serde::{Serialize, Deserialize};

use crate::get_now_naive_date_time;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_datasource_combination")]
#[sub_model(
    none(
        name = "CombinationId",
        extra(derive(sea_orm::FromQueryResult, Debug))
    ),
    all (
        name = "CombinationInfo",
        extra(derive(sea_orm::FromQueryResult, Debug, Serialize, Deserialize))
    )
)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sub_model(ignore("CombinationInfo"))]
    pub id: i32,
    #[sub_model(want("CombinationId"))]
    pub combination_id: String,
    pub bitmap1: u64,
    pub bitmap2: u64,
    pub bitmap3: u64,
    pub bitmap4: u64,
    #[sub_model(ignore("CombinationInfo"))]
    pub last_access_time: DateTime,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {}

impl CombinationInfo {
    pub fn into_active_model_with_access_time(self) -> ActiveModel {
        ActiveModel {
            combination_id: Set(self.combination_id),
            bitmap1: Set(self.bitmap1),
            bitmap2: Set(self.bitmap2),
            bitmap3: Set(self.bitmap3),
            bitmap4: Set(self.bitmap4),
            last_access_time: Set(get_now_naive_date_time()),
            ..Default::default()
        }
    }
}