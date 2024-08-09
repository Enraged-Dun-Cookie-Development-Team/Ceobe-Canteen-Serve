use axum::extract::Query;
use axum_resp_result::{MapReject, resp_result};
use tracing::instrument;
use persistence::ceobe_operate::models::version::models::ReleaseVersion;
use persistence::ceobe_operate::{ToCeobe, ToCeobeOperation};
use persistence::mongodb::MongoDatabaseOperate;
use persistence::operate::operate_trait::OperateTrait;
use crate::router::CdnOperationVersion;
use super::QueryReleaseVersion;
use super::{Result,MapRejecter};
impl CdnOperationVersion{
    #[resp_result]
    // TODO: 这里把挂载的东西一起带进去可能会好点？
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn release_version(
        db: MongoDatabaseOperate,
        MapReject(QueryReleaseVersion { version, platform }): MapRejecter<
            Query<QueryReleaseVersion>,
        >,
    ) -> Result<ReleaseVersion>
    {

        let release_info = match version {
            None => {
                db.ceobe()
                    .operation()
                    .release_version()
                    .retrieve()
                    .latest_by_platform(platform)
                    .await?
            }
            Some(ver) => {
                db.ceobe()
                    .operation()
                    .release_version()
                    .retrieve()
                    .by_version_platform(&ver, platform)
                    .await?
            }
        };

        Ok(release_info)
    }
}