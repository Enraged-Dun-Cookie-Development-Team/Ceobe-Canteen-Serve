use persistence::{ceobe_operate::{resource, ToCeobeOperation}, ceobe_user::ToCeobe, mysql::SqlDatabaseOperate};

use crate::{error::LogicResult, view::Resource};

use super::CeobeOperateLogic;



impl CeobeOperateLogic {
    /// 更新资源
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
    }

    /// 获取资源列表
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

