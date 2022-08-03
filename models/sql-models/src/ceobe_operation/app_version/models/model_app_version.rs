use sea_orm::{ entity::prelude::*, Set };

use crate::{get_zero_data_time, get_now_naive_date_time};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ceobe_operation_app_version")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
    /// field for soft delete
    pub(in crate::ceobe_operation::app_version) create_at: DateTime,
    pub(in crate::ceobe_operation::app_version) modify_at: DateTime,
    pub(in crate::ceobe_operation::app_version) delete_at: DateTime,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    // 软删除
    pub fn soft_remove(&mut self) {
        let now = get_now_naive_date_time();
        self.delete_at = Set(now);
    }

    // 还原删除
    pub fn soft_recover(&mut self) {
        self.delete_at = Set(get_zero_data_time())
    }

    // 更新操作
    pub fn now_modify(&mut self) {
        let now = get_now_naive_date_time();
        self.modify_at = Set(now);
    }
}