use checker::{CheckExtract, JsonCheckExtract};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::ceobe_operation::resource::{
        checkers::resource_data::CeobeOperationResourceChecker,
        operate::CeobeOperationResourceSqlOperate,
    },
};
use resp_result::{rtry, RespResult};

use super::{
    error::{ResourceError, ResourceRResult},
    view::Resource,
};
use crate::router::CeobeOpResource;

type ResourceUploadCheck =
    JsonCheckExtract<CeobeOperationResourceChecker, ResourceError>;

impl CeobeOpResource {
    pub async fn upload_resource(
        db: SqlConnect, CheckExtract(resource, _): ResourceUploadCheck,
    ) -> ResourceRResult<()> {
        CeobeOperationResourceSqlOperate::update_resource(&db, resource)
            .await
            .map_err(Into::into)
            .into()
    }

    pub async fn get_resource(db: SqlConnect) -> ResourceRResult<Resource> {
        let resp =
            CeobeOperationResourceSqlOperate::get_resource(&db, |raa, cd| {
                Resource::from((raa, cd))
            })
            .await;

        RespResult::ok(rtry!(resp))
    }
}
