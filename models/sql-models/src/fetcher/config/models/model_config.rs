use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fetcher_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 存活个数情况
    pub live_number: i8,
    /// 当前存活个数下，第n个蹲饼器
    pub fetcher_count: i8,
    /// 蹲饼组名字
    pub group_name: String,
    /// 蹲饼相关平台
    pub platform: String,
    /// 数据源id
    pub datasource_id: i32,
    /// 组内蹲饼间隔
    pub interval: Option<u64>,
    /// 组内蹲饼时间段间隔
    pub interval_by_time_range: String,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}
