use axum_resp_result::{resp_result, MapReject};
use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use checker::SerdeCheck;
use page_size::response::ListWithPageInfo;
use persistence::ceobe_operate::models::version::models::{
    ReleasePlatform, ReleaseVersion,
};
use serve_utils::{
    axum::extract::Query, FetchViewValue, FetchOptionViewValue, OptionViewField,
};
use tracing::instrument;

use super::{MapRejecter, Result};
use crate::{view::QueryVersionFilter, ReleaseVersionController};

impl ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all)]
    pub async fn all_version<
        D: OptionViewField<bool> + FetchViewValue<bool>,
        P: OptionViewField<ReleasePlatform>
            + FetchOptionViewValue<ReleasePlatform>,
    >(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryVersionFilter::<D, P> {
            platform,
            deleted,
            paginator: SerdeCheck(paginator),
        }): MapRejecter<Query<QueryVersionFilter<D, P>>>,
    ) -> Result<ListWithPageInfo<ReleaseVersion>> {
        let ret = logic
            .all(paginator.into(), platform.fetch_option(), deleted.fetch())
            .await?;

        Ok(ret)
    }
}
