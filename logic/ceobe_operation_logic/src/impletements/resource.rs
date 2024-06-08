use persistence::{ceobe_operate::{resource, ToCeobeOperation}, ceobe_user::ToCeobe, mysql::SqlDatabaseOperate};
use tencent_cloud_server::cloud_manager::CloudManager;

use crate::{error::LogicResult, view::Resource};

use super::CeobeOperateLogic;



impl CeobeOperateLogic {
    /// 更新资源
    pub async fn upload_resource(
        sql: SqlDatabaseOperate, tc_cloud: CloudManager, resource: resource::Checked
    ) -> LogicResult<()> {
        sql
            .ceobe()
            .operation()
            .resource()
            .update_resource(resource)
            .await?;
        
        let paths = vec!["/cdn/operate/resource/get"];
        tc_cloud.purge_urls_cache(paths).await?;

        Ok(())
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

