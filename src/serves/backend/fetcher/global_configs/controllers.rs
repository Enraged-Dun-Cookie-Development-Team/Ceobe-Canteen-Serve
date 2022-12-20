use axum::{body::Body, Json};
use checker::{CheckExtract, JsonCheckExtract};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::fetcher::global_config::checkers::global_config_data::{
        FetcherGlobalConfig, FetcherGlobalConfigChecker,
    },
};
use serde_json::{Map, Value};
use tracing::instrument;
use resp_result::MapReject;
use crate::router::FetcherConfigControllers;

use super::error::{GlobalConfigError, GlobalConfigRResult};

type FetcherGlobalConfigUploadCheck =
    JsonCheckExtract<FetcherGlobalConfigChecker, GlobalConfigError>;

impl FetcherConfigControllers {
    #[instrument(ret, skip(db))]
    pub async fn upload_global_config(
        db: SqlConnect, 
        MapReject(global_config): MapReject<Json<Map<String, Value>>, GlobalConfigError>,
    ) -> GlobalConfigRResult<()> {
        fetcher_logic::implement::set_global_config(&db, global_config)
            .await
            .map_err(Into::into)
            .into()
    }
}
