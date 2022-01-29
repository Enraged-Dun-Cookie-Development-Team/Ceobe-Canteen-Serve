//! SeaORM Entity. Generated by sea-orm-codegen 0.4.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "each_mansion")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub id: i64,
    pub mid: i64,
    #[sea_orm(unique)]
    pub date: Date,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::mansion::Entity",
        from = "Column::Mid",
        to = "super::mansion::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Mansion,
    #[sea_orm(has_many = "super::mansion_info::Entity")]
    MansionInfo,
}

impl Related<super::mansion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mansion.def()
    }
}

impl Related<super::mansion_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MansionInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
