use db_ops_prelude::sea_orm::{IntoActiveModel, ActiveModelTrait, EntityTrait};
use db_ops_prelude::sql_models::ceobe_operation::tool_link;
use db_ops_prelude::{get_connect::GetDatabaseConnect, sea_orm::ConnectionTrait, sql_models::ceobe_operation::tool_link::checkers::tool_link_data::CeobeOperationToolLink};
use tracing::{instrument, info};
use super::OperateResult;


use super::ToolLinkOperate;

impl<'c, C> ToolLinkOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self), ret)]
    pub async fn delete_one(
        &'c self, id: i32,
    ) -> OperateResult<()> {
        info!(tool_link.id = id);
        let db = self.get_connect();

        tool_link::models::model_tool_link::Entity::delete_by_id(id).exec(db).await?;

        Ok(())
    }
}