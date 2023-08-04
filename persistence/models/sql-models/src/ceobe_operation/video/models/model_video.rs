use sea_orm::{entity::prelude::*, ActiveValue};
use sea_orm::ActiveValue::Set;
use sql_connection::database_traits::has_scheme::Has;
use sql_connection::ext_traits::with_field::FieldOrder;
use crate::{NaiveDateTime, SoftDelete};

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

impl SoftDelete for ActiveModel {
    fn get_mut(&mut self) -> &mut ActiveValue<NaiveDateTime> {
        &mut self.delete_at
    }
}

impl Has<FieldOrder>  for ActiveModel{
    type Ty = i32;

    fn set(&mut self, value: Self::Ty) {
        self.order = Set(value)
    }
}