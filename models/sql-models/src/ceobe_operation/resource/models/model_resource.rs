use chrono::Local;
use sea_orm::{entity::prelude::*, FromQueryResult, Set};
use sub_model::SubModel;

use super::resource_type::ResourceType;
use crate::get_zero_data_time;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "ceobe_operation_resource")]
#[sub_model(
    none(
        vis = "pub(in crate::ceobe_operation::resource)",
        name = "ResourceAllAvailable",
        extra(derive(Debug, Clone, FromQueryResult))
    ),
    all(
        vis = "pub(in crate::ceobe_operation::resource)",
        name = "Countdown",
        extra(derive(Debug, Clone, FromQueryResult))
    )
)]
pub struct Model {
    #[sub_model(ignore("Countdown"))]
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sub_model(ignore("Countdown"))]
    pub(in crate::ceobe_operation::resource) ty: ResourceType,
    // data
    pub message: String,
    pub banner_info: String,
    pub countdown_end: DateTime,
    #[sub_model(want("ResourceAllAvailable",))]
    pub start_time: DateTime,
    #[sub_model(want("ResourceAllAvailable",))]
    pub over_time: DateTime,

    /// field for soft delete
    #[sub_model(want("ResourceAllAvailable",))]
    pub(in crate::ceobe_operation::resource) create_at: DateTime,
    #[sub_model(want("ResourceAllAvailable",))]
    pub(in crate::ceobe_operation::resource) modify_at: DateTime,
    #[sub_model(want("ResourceAllAvailable",))]
    pub(in crate::ceobe_operation::resource) delete_at: DateTime,
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

    pub fn now_create_with_time(&mut self, now: chrono::NaiveDateTime) {
        self.create_at = Set(now);
        self.modify_at = Set(now)
    }

    // 更新操作
    pub fn now_modify(&mut self) {
        let now = Local::now().naive_local();
        self.modify_at = Set(now);
    }
}
