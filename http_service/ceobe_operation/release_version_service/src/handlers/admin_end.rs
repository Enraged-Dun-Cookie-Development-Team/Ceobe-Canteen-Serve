use persistence::{
    ceobe_operate::{ToCeobe, ToCeobeOperation},
    mongodb::MongoDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use serve_utils::{
    axum::extract::Query,
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    ValueField,
};

use crate::{
    handlers::{MapRejecter, Result},
    view::QueryReleaseVersion,
};

impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn yank_version(
        db: MongoDatabaseOperate,
        MapReject(QueryReleaseVersion {
            version: ValueField(version),
            platform,
        }): MapRejecter<
            Query<QueryReleaseVersion<ValueField<semver::Version>>>,
        >,
    ) -> Result<()> {
        db.ceobe()
            .operation()
            .release_version()
            .delete()
            .yank(&platform, &version)
            .await?;
        Ok(())
    }
}
