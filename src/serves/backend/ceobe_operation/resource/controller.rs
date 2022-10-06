use axum_prehandle::{
    prefabs::json::JsonPayload, PreHandling, PreRespHandling,
};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::ceobe_operation::resource::{
        checkers::resource_data::{
            CeobeOperationResourceChecker, CeobeOperationResourceUncheck,
        },
        operate::CeobeOperationResourceSqlOperate,
    },
};
use resp_result::RespResult;

use super::{
    error::{ResourceError, ResourceRResult},
    view::Resource,
};
use crate::{router::CeobeOpResource, utils::data_checker::PreLiteChecker};

type ResourceUploadCheck = PreLiteChecker<
    JsonPayload<CeobeOperationResourceUncheck>,
    CeobeOperationResourceChecker,
    ResourceError,
>;

impl CeobeOpResource {
    pub async fn upload_resource(
        db: SqlConnect,
        PreHandling(resource): PreRespHandling<ResourceUploadCheck>,
    ) -> ResourceRResult<()> {
        CeobeOperationResourceSqlOperate::update_resource(&db, resource)
            .await?;
        RespResult::ok(())
    }

    pub async fn get_resource(db: SqlConnect) -> ResourceRResult<Resource> {
        let resp =
            CeobeOperationResourceSqlOperate::get_resource(&db, |raa, cd| {
                Resource::from((raa, cd))
            })
            .await?;

        RespResult::ok(resp)
    }
}
