use axum::Json;
use orm_migrate::sql_connection::SqlConnect;
use resp_result::{rtry, MapReject, RespResult};
use serde_json::{Map, Value};
use tracing::instrument;

use super::error::{GlobalConfigError, GlobalConfigRResult};
use crate::router::FetcherConfigControllers;

impl FetcherConfigControllers {
    // 更新蹲饼器全局变量
    #[instrument(ret, skip(db))]
    pub async fn upload_global_configs(
        db: SqlConnect,
        MapReject(global_config): MapReject<
            Json<Map<String, Value>>,
            GlobalConfigError,
        >,
    ) -> GlobalConfigRResult<()> {
        fetcher_logic::implement::set_global_config(&db, global_config)
            .await
            .map_err(Into::into)
            .into()
    }

    // 获取蹲饼器全局变量
    #[instrument(ret, skip(db))]
    pub async fn get_global_configs(
        db: SqlConnect,
    ) -> GlobalConfigRResult<Value> {
        let resp = fetcher_logic::implement::get_global_configs(&db).await;
        RespResult::ok(rtry!(resp))
    }
}
