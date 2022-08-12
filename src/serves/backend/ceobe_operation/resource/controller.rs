use std::time::Duration;

use axum::headers::HeaderName;
use axum_prehandle::{
    prefabs::json::JsonPayload, PreHandling, PreRespHandling,
};
use modify_cache::{CacheMode, CheckModify};
use orm_migrate::sql_models::ceobe_operation::resource::{
    checkers::resource_data::{
        CeobeOperationResourceChecker, CeobeOperationResourceUncheck,
    },
    operate::CeobeOperationResourceSqlOperate,
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
        PreHandling(resource): PreRespHandling<ResourceUploadCheck>,
    ) -> ResourceRResult<()> {
        CeobeOperationResourceSqlOperate::update_resource(resource).await?;
        RespResult::ok(())
    }

    pub async fn get_resource() -> ResourceRResult<Resource> {
        let resp =
            CeobeOperationResourceSqlOperate::get_resource(|raa, cd| {
                Resource::from((raa, cd))
            })
            .await?;

        RespResult::ok(data)
    }
}
