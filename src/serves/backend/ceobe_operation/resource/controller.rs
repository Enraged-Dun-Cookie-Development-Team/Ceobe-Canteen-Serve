use checker::{CheckExtract, JsonCheckExtract};
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::{resource, ToSqlCeobeOperation},
};
use resp_result::{rtry, RespResult};
use tracing::instrument;

use super::{
    error::{ResourceError, ResourceRResult},
    view::Resource,
};
use crate::router::CeobeOpResource;

type ResourceUploadCheck = JsonCheckExtract<resource::Checker, ResourceError>;

impl CeobeOpResource {
    #[instrument(ret, skip(db))]
    pub async fn upload_resource(
        db: SqlDatabaseOperate,
        CheckExtract(resource): ResourceUploadCheck,
    ) -> ResourceRResult<()> {
        db.ceobe_operation()
            .resource()
            .update_resource(resource)
            .await
            .map_err(Into::into)
            .into()
    }

    #[instrument(ret, skip(db))]
    pub async fn get_resource(
        db: SqlDatabaseOperate,
    ) -> ResourceRResult<Resource> {
        let resp = db
            .ceobe_operation()
            .resource()
            .get(|raa, cd| Resource::from((raa, cd)))
            .await;

        RespResult::ok(rtry!(resp))
    }
}
