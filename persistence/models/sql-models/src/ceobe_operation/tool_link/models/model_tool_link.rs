use sea_orm::{entity::prelude::*, ActiveValue};

use crate::{NaiveDateTime, SoftDelete};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ceobe_operation_tool_link")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nickname: String,
    pub avatar: String,
    pub jump_url: String,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}