use persistence::{
    ceobe_operate::{resource, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use persistence::ceobe_operate::ToCeobe;
use tencent_cloud_server::{
    cdn::purge_urls_cache::PurgeCachePath, cloud_manager::TencentCloudManager,
};

use super::CeobeOperateLogic;
use crate::{
    error::LogicResult,
    view::{OperationTcCdnPath, Resource},
};

impl CeobeOperateLogic {
    /// 更新资源
    pub async fn upload_resource(
        sql: SqlDatabaseOperate, tc_cloud: TencentCloudManager,
        resource: resource::Checked,
    ) -> LogicResult<()> {
        sql.ceobe()
            .operation()
            .resource()
            .update_resource(resource)
            .await?;

        const PATHS: [PurgeCachePath; 1] =
            [OperationTcCdnPath::RESOURCE_LIST_PATH];
        tc_cloud.purge_urls_cache(&PATHS).await?;

        Ok(())
    }

    /// 获取资源列表
    pub async fn get_resource(
        sql: SqlDatabaseOperate,
    ) -> LogicResult<Resource> {
        Ok(sql
            .ceobe()
            .operation()
            .resource()
            .get(|raa, cd| Resource::from((raa, cd)))
            .await?)
    }
}
