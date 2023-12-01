use sea_orm::entity::prelude::*;
use sub_model::SubModel;

#[derive(
    Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel, serde::Serialize,
)]
#[sea_orm(table_name = "ceobe_operation_tool_link")]
#[sub_model(all(
    name = "FrontendToolLink",
    extra(derive(sea_orm::FromQueryResult, serde::Serialize, Debug))
))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sub_model(ignore("FrontendToolLink"))]
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
