use sea_orm::entity::prelude::*;
use sub_model::SubModel;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SubModel)]
#[sea_orm(table_name = "ceobe_cookie_search_content")]
#[sub_model(none(
    name = "SearchOid",
    extra(derive(sea_orm::FromQueryResult, Debug))
))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sub_model(want("SearchOid"))]
    pub object_id: String,
    pub source_config_id: i32,
    pub content: String,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {}
