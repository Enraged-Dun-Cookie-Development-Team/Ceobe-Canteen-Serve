use checker::{CheckExtract, JsonCheckExtract};
use database_traits::database_operates::sub_operate::SuperOperate;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::{
        resource::{
            checkers::resource_data::CeobeOperationResourceChecker,
            operate::ResourceOperate,
        },
        SqlCeobeOperation,
    },
};
use resp_result::{rtry, RespResult};
use tracing::instrument;

use super::{
    error::{ResourceError, ResourceRResult},
    view::Resource,
};
use crate::router::CeobeOpResource;

type ResourceUploadCheck =
    JsonCheckExtract<CeobeOperationResourceChecker, ResourceError>;

impl CeobeOpResource {
    #[instrument(ret, skip(db))]
    pub async fn upload_resource(
        mut db: SqlDatabaseOperate,
        CheckExtract(resource): ResourceUploadCheck,
    ) -> ResourceRResult<()> {
        db.child::<SqlCeobeOperation<_>>()
            .child::<ResourceOperate<_>>()
            .update_resource(resource)
            .await
            .map_err(Into::into)
            .into()
    }

    #[instrument(ret, skip(db))]
    pub async fn get_resource(
        mut db: SqlDatabaseOperate,
    ) -> ResourceRResult<Resource> {
        let resp = db
            .child::<SqlCeobeOperation<_>>()
            .child::<ResourceOperate<_>>()
            .get(|raa, cd| Resource::from((raa, cd)))
            .await;

        RespResult::ok(rtry!(resp))
    }
}
