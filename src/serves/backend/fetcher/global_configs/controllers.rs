use axum::Json;
use fetcher_logic::implements::{FetcherConfigLogic, SuperLogic, GlobalConfig};
use orm_migrate::sql_connection::SqlConnect;
use resp_result::MapReject;
use serde_json::{Map, Value};
use tracing::instrument;

use super::error::{GlobalConfigError, GlobalConfigRResult};
use crate::router::FetcherConfigControllers;

impl FetcherConfigControllers {
    /// 更新蹲饼器全局变量
    #[instrument(ret, skip(db))]
    pub async fn upload_global_configs(
        db: SqlConnect,
        MapReject(global_config): MapReject<
            Json<Map<String, Value>>,
            GlobalConfigError,
        >,
    ) -> GlobalConfigRResult<()> {
        FetcherConfigLogic
            .sub_logic::<GlobalConfig>()
            .set(&db, global_config)
            .await
            .map_err(Into::into)
            .into()
    }

    /// 获取蹲饼器全局变量
    #[instrument(ret, skip(db))]
    pub async fn get_global_configs(
        db: SqlConnect,
    ) -> GlobalConfigRResult<Value> {
        FetcherConfigLogic
            .sub_logic::<GlobalConfig>()
            .get_all(&db)
            .await
            .map_err(Into::into)
            .into()
    }
}
