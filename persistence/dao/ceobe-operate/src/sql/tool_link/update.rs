use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel},
    sql_models::ceobe_operation::tool_link::checkers::tool_link_data::CeobeOperationToolLink,
};
use tracing::{info, instrument};

use super::{OperateResult, ToolLinkOperate};
use crate::tool_link::OperateError;

impl<'c, C> ToolLinkOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self))]
    /// 分页获取用户列表
    pub async fn update_one(
        &'c self, tool_link: CeobeOperationToolLink,
    ) -> OperateResult<()> {
        info!(
            toolLink.id = tool_link.id,
            toolLink.name = tool_link.nickname,
            toolLink.avatar = tool_link.avatar.to_string(),
            toolLink.jump_url = tool_link.jump_url.to_string()
        );

        let db = self.get_connect();

        if ToolLinkOperate::is_id_exist(db, tool_link.id.unwrap()).await? {
            tool_link.into_active_model().update(db).await?;
        }
        else {
            return Err(OperateError::ToolLinkNotFound(
                tool_link.id.unwrap(),
            ));
        };

        Ok(())
    }
}
