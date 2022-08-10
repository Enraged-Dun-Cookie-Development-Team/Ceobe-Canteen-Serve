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
use resp_result::{FlagWarp, RespResult};

use super::{
    error::{ResourceError, ResourceRResult},
    view_models::Resource,
};
use crate::{
    router::CeobeOpResource,
    utils::{
        data_checker::PreLiteChecker, user_authorize::config::get_header_name,
    },
};

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

    pub async fn get_resource(
        mut modify: CheckModify,
    ) -> ResourceRResult<FlagWarp<Option<Resource>>> {
        modify
            .cache_headers
            .clean_content_local()
            .add_vary_headers([HeaderName::from_static(get_header_name())])
            .get_control()
            .set_ty(CacheMode::Private)
            .set_max_age(Duration::from_secs(4 * 60 * 60));

        let resp =
            CeobeOperationResourceSqlOperate::get_resource(|raa, cd| {
                Resource::from((raa, cd))
            })
            .await?;

        let (data, flags) = modify.check_modify(resp)?;

        RespResult::ok(data).with_flags(flags)
    }
}
