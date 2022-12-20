
use chrono::Local;
use sea_orm::{ entity::prelude::*, Set };

use crate::get_zero_data_time;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fetcher_platform_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    /// field for soft delete
    pub(in crate::fetcher::platform_config) create_at: DateTime,
    pub(in crate::fetcher::platform_config) modify_at: DateTime,
    pub(in crate::fetcher::platform_config) delete_at: DateTime,
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
        let now = Local::now().naive_local();
        self.delete_at = Set(now);
    }

    // 还原删除
    pub fn soft_recover(&mut self) {
        self.delete_at = Set(get_zero_data_time())
    }

    // 新建操作
    pub fn now_create(&mut self) {
        let now = Local::now().naive_local();
        self.create_at = Set(now);
        self.modify_at = Set(now);
    }

    // 更新操作
    pub fn now_modify(&mut self) {
        let now = Local::now().naive_local();
        self.modify_at = Set(now);
    }
}
