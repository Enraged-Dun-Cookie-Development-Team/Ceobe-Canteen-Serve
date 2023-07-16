use axum::extract::{Path, State};
use bitmap_convert::{
    base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv,
};
use bitmaps::Bitmap;
use fetcher::{datasource_config::ToDatasource, ToFetcher};
use general_request_client::{client::RequestClient, traits::Requester};
use http::Method;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::fetcher::datasource_config::models::model_datasource_config::FrontendDatasource,
};
use resp_result::{resp_try, rtry};
use scheduler_notifier::SchedulerUrl;
use serde_json::{json, Value};
use tracing::instrument;
use url::Url;

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

    #[instrument(skip(request_client))]
    // #[axum_macros::debug_handler]
    pub async fn standalone_fetcher_config(
        Path(combine_id): Path<String>,
        State(request_client): State<RequestClient>,
        State(url): State<SchedulerUrl>,
    ) -> DatasourceRResult<Value> {
        struct StandAloneFetcherConfigRequester {
            url: SchedulerUrl,
            bitmap: Vec<usize>,
        }

        impl Requester for StandAloneFetcherConfigRequester {
            const METHOD: http::Method = Method::POST;

            fn get_url(&self) -> Url { self.url.take_url() }

            fn prepare_request<
                B: general_request_client::traits::RequestBuilder,
            >(
                self, builder: B,
            ) -> Result<B::Request, B::Error> {
                builder
                    .body(
                        json!({"datasource_id_list":self.bitmap})
                            .to_string()
                            .into_bytes(),
                    )
                    .build()
            }
        }

        resp_try(async {
            let bitmap = Bitmap::<256>::from_base_70(combine_id)
                .map(|bitmap| bitmap.bitmap_to_usize())?;

            let resp = request_client
                .send_request(StandAloneFetcherConfigRequester {
                    url,
                    bitmap,
                })
                .await?;
            #[derive(serde::Deserialize)]
            struct ConfigPayload {
                #[serde(rename = "code")]
                _code: i32,
                config: Value,
            }
            let ConfigPayload { config, .. } =
                resp.json::<ConfigPayload>().await?;
            Ok(config)
        })
        .await
    }
}
