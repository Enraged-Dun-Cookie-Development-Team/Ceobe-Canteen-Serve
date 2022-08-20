use sea_orm::{entity::prelude::*, Set};

use crate::{get_now_naive_date_time, get_zero_data_time};

#[derive(Debug, Clone, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ceobe_operation_video")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub bv: String,
    pub order: i32,
    pub start_time: DateTime,
    pub over_time: DateTime,
    pub title: String,
    pub author: String,
    pub video_link: String,
    pub cover_image: String,
    /// field for soft delete
    pub(in crate::ceobe_operation::video) delete_at: DateTime,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn soft_remove(&mut self) {
        self.delete_at = Set(get_now_naive_date_time());
    }

    pub fn soft_recover(&mut self) {
        self.delete_at = Set(get_zero_data_time())
    }
}
