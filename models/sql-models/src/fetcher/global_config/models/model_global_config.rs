use sea_orm::entity::prelude::*;

// TODO: 多存个类型吧 挺好
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fetcher_global_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub key: String, // 键
    pub value: String,  // 值
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {}
