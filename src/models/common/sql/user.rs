use sea_orm::entity::prelude::*;
use helper_macro::entity;

use super::auth::Auth;

#[entity]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique, column_type = "String(Some(16))")]
    pub username: String,
    #[sea_orm(column_type = "Char(Some(64))")]
    pub password: String,

    pub auth: Auth,
    #[sea_orm(default_value = 0)]
    pub num_pwd_change: u32,
}

#[derive(Debug, Clone, Copy, EnumIter,DeriveRelation)]
pub enum Relation {}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef { panic!("No Relation") }
// }

impl ActiveModelBehavior for ActiveModel {}
