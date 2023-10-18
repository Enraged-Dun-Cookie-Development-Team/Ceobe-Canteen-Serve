use db_ops_prelude::{database_operates::NoConnect, sea_orm::{ConnectionTrait, ColumnTrait, EntityTrait, QueryFilter}, sql_models::ceobe_operation::tool_link::models::model_tool_link::{Column, Entity}, ext_traits::select_count::QueryCountByColumn};
use crate::tool_link::OperateResult;
use tracing::instrument;

use super::ToolLinkOperate;

impl ToolLinkOperate<'_, NoConnect> {
    #[instrument(skip(db), ret)]
    /// 是否存在该工具链接
    pub async fn is_id_exist(
        db: &impl ConnectionTrait, did: i32,
    ) -> OperateResult<bool> {
        Ok(Entity::find()
            .filter(Column::Id.eq(did))
            .count_non_zero_by_column(Column::Id)
            .one(db)
            .await?
            .unwrap()
            .take())
    }
}