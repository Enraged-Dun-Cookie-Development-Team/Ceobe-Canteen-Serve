use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel},
    sql_models::ceobe_operation::tool_link::checkers::tool_link_data::CeobeOperationToolLink,
};
use tracing::{info, instrument};

use super::{OperateResult, ToolLinkOperate};

impl<'c, C> ToolLinkOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self, tool_link), ret)]
    pub async fn create_one(
        &'c self, tool_link: CeobeOperationToolLink,
    ) -> OperateResult<()> {
        info!(tool_link.create = ?tool_link);

        let db = self.get_connect();
        tool_link.into_active_model().insert(db).await?;

        Ok(())
    }
}
