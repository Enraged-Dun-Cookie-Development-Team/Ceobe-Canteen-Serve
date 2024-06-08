use persistence::{ceobe_operate::{resource, ToCeobeOperation}, ceobe_user::ToCeobe, mysql::SqlDatabaseOperate};

use crate::{error::LogicResult, view::Resource};

use super::CeobeOperateLogic;



impl CeobeOperateLogic {
    /// 获取公告列表
    pub async fn upload_resource(
        sql: SqlDatabaseOperate, resource: resource::Checked
    ) -> LogicResult<()> {
        sql
            .ceobe()
            .operation()
            .resource()
            .update_resource(resource)
            .await
            .map_err(Into::into)
            .into()
    }

     /// 更新公告
    pub async fn get_resource(
        sql: SqlDatabaseOperate
    ) -> LogicResult<Resource> {
        Ok(sql.ceobe()
            .operation()
            .resource()
            .get(|raa, cd| Resource::from((raa, cd)))
            .await?)
    }
}

