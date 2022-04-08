use sea_orm::entity::prelude::*;
use helper_macro::entity;
use sub_model_derive::SubModel;

use super::auth::Auth;

#[entity]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel,SubModel)]
#[sea_orm(table_name = "user")]
#[sub_model(all("VerifiedUser"))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique, column_type = "String(Some(16))")]
    pub username: String,
    #[sea_orm(column_type = "Char(Some(64))")]
    pub password: String,
    #[sub_model(field(ignore(name="VerifiedUser")))]
    pub auth: Auth,
}

#[derive(Debug, Clone, Copy, EnumIter,DeriveRelation)]
pub enum Relation {}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef { panic!("No Relation") }
// }

impl ActiveModelBehavior for ActiveModel {}
