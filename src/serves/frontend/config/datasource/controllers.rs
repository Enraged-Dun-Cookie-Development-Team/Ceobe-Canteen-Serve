use fetcher::{datasource_config::ToDatasource, ToFetcher};
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::fetcher::datasource_config::models::model_datasource_config::FrontendDatasource,
};
use resp_result::rtry;
use tracing::instrument;

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
