use persistence::fetcher::{datasource_config::ToDatasource, ToFetcher};

use resp_result::rtry;
use tracing::instrument;
use persistence::fetcher::models::datasource_config::models::model_datasource_config::FrontendDatasource;
use persistence::mysql::SqlDatabaseOperate;

use crate::{
    router::ConfigDatasourceFrontend,
    serves::frontend::config::datasource::error::DatasourceRResult,
};

impl ConfigDatasourceFrontend {
    /// 获取平台与数据源类型列
    #[instrument(skip(db))]
    pub async fn datasource_list(
        db: SqlDatabaseOperate,
    ) -> DatasourceRResult<Vec<FrontendDatasource>> {
        Ok(rtry!(
            db.fetcher().datasource().find_all_with_unique_id().await
        ))
        .into()
    }
}
