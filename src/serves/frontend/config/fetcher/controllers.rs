use axum::extract::{Path, State};
use bitmap_convert::{
    base70::BitmapBase70Conv, vec_usize::BitmapVecUsizeConv,
};
use bitmaps::Bitmap;
use general_request_client::{client::RequestClient, traits::Requester};
use http::{Method, Version};
use resp_result::resp_try;
use scheduler_notifier::SchedulerUrl;
use serde_json::{json, Value};
use tap::Tap;
use tracing::instrument;
use url::Url;

use super::error::FetcherRResult;
use crate::router::ConfigFetcherFrontend;

impl ConfigFetcherFrontend {
    #[instrument(skip(request_client))]
    pub async fn standalone_fetcher_config(
        Path(combine_id): Path<String>,
        State(request_client): State<RequestClient>,
        State(url): State<SchedulerUrl>,
    ) -> FetcherRResult<Value> {
        struct StandAloneFetcherConfigRequester {
            url: SchedulerUrl,
            bitmap: Vec<usize>,
        }

        impl Requester for StandAloneFetcherConfigRequester {
            const METHOD: Method = Method::POST;
            const VERSION: Version = Version::HTTP_11;

            fn get_url(&self) -> Url {
                self.url.take_url().tap_mut(|url| {
                    url.set_path("/standalone-fetcher-get-config")
                })
            }

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
