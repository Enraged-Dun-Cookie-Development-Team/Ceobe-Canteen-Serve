use chrono::Local;
use sea_orm::{entity::prelude::*, Set};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ceobe_operation_announcement")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub start_time: DateTime,
    pub over_time: DateTime,
    pub content: String,
    pub img_url: String,
    pub order: i32,
    pub notice: bool,
    /// field for soft delete
    pub(in crate::ceobe_operation::announcement) delete_at: DateTime,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    // 软删除
    pub fn soft_remove(&mut self) {
        let now = Local::now().naive_local();
        self.delete_at = Set(now);
    }

    // 还原删除
    pub fn soft_recover(&mut self) {
        let date_time = chrono::NaiveDateTime::from_timestamp(0, 0);
        self.delete_at = Set(date_time)
    }
}
