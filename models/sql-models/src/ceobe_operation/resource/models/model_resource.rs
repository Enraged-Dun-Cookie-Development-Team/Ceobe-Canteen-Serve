use chrono::Local;
use sea_orm::{entity::prelude::*, ActiveValue, FromQueryResult, Set};
use sub_model::SubModel;

use super::resource_type::ResourceType;
use crate::{NaiveDateTime, SoftDelete};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "ceobe_operation_resource")]
#[sub_model(
    none(
        name = "ResourceAllAvailable",
        extra(derive(Debug, Clone, FromQueryResult))
    ),
    all(name = "Countdown", extra(derive(Debug, Clone, FromQueryResult)))
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

impl SoftDelete for ActiveModel {
    fn get_mut(&mut self) -> &mut ActiveValue<NaiveDateTime> {
        &mut self.delete_at
    }
}
