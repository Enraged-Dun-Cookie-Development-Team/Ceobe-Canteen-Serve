//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mansion")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i64,
    pub mid: i32,
    pub sub_mid: i32,
    pub create_time: DateTime,
    pub edit_time: DateTime,
    pub link: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::each_mansion::Entity")]
    EachMansion,
}

impl Related<super::each_mansion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EachMansion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
