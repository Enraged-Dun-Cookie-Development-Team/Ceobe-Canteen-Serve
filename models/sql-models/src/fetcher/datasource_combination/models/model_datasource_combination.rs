use sea_orm::{entity::prelude::*, Set};
use sub_model::SubModel;

use crate::get_now_naive_date_time;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "fetcher_datasource_combination")]
#[sub_model(none(
    name = "CombinationId",
    extra(derive(sea_orm::FromQueryResult, Debug))
))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sub_model(want("CombinationId"))]
    pub combination_id: String,
    pub bitmap1: u64,
    pub bitmap2: u64,
    pub bitmap3: u64,
    pub bitmap4: u64,
    pub last_access_time: DateTime,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(combination_id: String, [bitmat1,bitmat2,bitmap3,bitmap4]: [u64; 4]) -> ActiveModel {
        ActiveModel {
            combination_id: Set(combination_id),
            bitmap1: Set(bitmat1),
            bitmap2: Set(bitmat2),
            bitmap3: Set(bitmap3),
            bitmap4: Set(bitmap4),
            last_access_time: Set(get_now_naive_date_time()),
            ..Default::default()
        }
    }
}
