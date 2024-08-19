use persistence::{
    ceobe_operate::{
        models::version::models::ReleaseVersion, ToCeobe, ToCeobeOperation,
    },
    mongodb::MongoDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use serve_utils::{
    axum::extract::Query,
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    OptionValueField,
};

use super::{MapRejecter, Result};
use crate::view::QueryReleaseVersion;
impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn release_version(
        db: MongoDatabaseOperate,
        MapReject(QueryReleaseVersion {
            version: OptionValueField(version),
            platform,
        }): MapRejecter<Query<QueryReleaseVersion>>,
    ) -> Result<ReleaseVersion> {
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
