use sea_orm::entity::prelude::*;
use sub_model::SubModel;

use super::auth_level::AuthLevel;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "admin_user")]
#[sub_model(
    all(name = "VerifiedUser", extra(derive(sea_orm::FromQueryResult))),
    none(name = "UserList", extra(derive(sea_orm::FromQueryResult)))
)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sub_model(want("UserList"))]
    pub id: i32,

    #[sea_orm(unique, column_type = "String(Some(16))")]
    #[sub_model(want("UserList"))]
    pub username: String,
    #[sea_orm(column_type = "Char(Some(64))")]
    pub password: String,
    #[sub_model(ignore("VerifiedUser"), want("UserList"))]
    pub auth: AuthLevel,
    #[sea_orm(default_value = 0)]
    pub num_pwd_change: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
