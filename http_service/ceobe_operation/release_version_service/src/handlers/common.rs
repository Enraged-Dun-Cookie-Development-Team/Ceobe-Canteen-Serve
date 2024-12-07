use axum_resp_result::{resp_result, MapReject};
use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use checker::SerdeCheck;
use page_size::response::ListWithPageInfo;
use persistence::ceobe_operate::models::version::models::ReleaseVersion;
use serve_utils::{
    axum::extract::Query, FetchViewValue, OptionField, OptionViewField,
};
use tracing::instrument;

use super::{MapRejecter, Result};
use crate::{view::QueryVersionFilter, ReleaseVersionController};

impl ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all)]
    pub async fn all_version<
        D: OptionViewField<bool> + FetchViewValue<bool>,
    >(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryVersionFilter::<D> {
            platform: OptionField(platform),
            deleted,
            paginator: SerdeCheck(paginator),
        }): MapRejecter<Query<QueryVersionFilter<D>>>,
    ) -> Result<ListWithPageInfo<ReleaseVersion>> {
        let ret = logic
            .all(paginator.into(), platform, deleted.fetch())
            .await?;

        Ok(ret)
    }
}
