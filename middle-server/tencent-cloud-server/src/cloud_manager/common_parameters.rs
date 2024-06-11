use general_request_client::HeaderValue;
use secrecy::ExposeSecret;
use serde::Serialize;
use url::Url;

use super::cloud_manager::TencentCloudManager;
use crate::{
    cloud_manager::{
        entities::{CommonParameter, RequestContent, TencentCloudResponse},
        signature::sign,
    },
    error::TcCloudError,
    requester::TencentCloudRequester,
};



impl TencentCloudManager {
    /// 通用请求
    pub(crate) async fn common_request<P: Serialize, Q: Serialize + Clone>(
        &self, common_params: &CommonParameter,
        request: &RequestContent<P, Q>,
    ) -> Result<TencentCloudResponse, TcCloudError> {
        let url =
            format!("https://{}.tencentcloudapi.com", common_params.service)
                .parse()?;
        let authorization = sign(
            self.id.expose_secret(),
            self.key.expose_secret(),
            common_params,
            request,
            &url,
        )?;

        let mut payload_buffer = Vec::<u8>::new();
        serde_json::to_writer(&mut payload_buffer, &request.payload)?;

        let requester = TencentCloudRequester::builder()
            .url(url.clone())
            .method(request.method.clone())
            .query(request.query.clone())
            .payload(payload_buffer)
            .host(HeaderValue::from_str(url.host_str().unwrap())?)
            .action(HeaderValue::from_str(common_params.action)?)
            .version(HeaderValue::from_str(common_params.version)?)
            .timestamp(HeaderValue::from_str(
                &common_params.timestamp.to_string(),
            )?)
            .content_type(HeaderValue::from_str(
                request.content_type.as_ref(),
            )?)
            .authorization(HeaderValue::from_str(&authorization)?)
            .region(common_params.region.clone().and_then(|region| {
                HeaderValue::from_str(&region)
                    .map_err(TcCloudError::from)
                    .ok()
            }))
            .token(common_params.token.clone().and_then(|token| {
                HeaderValue::from_str(&token)
                    .map_err(TcCloudError::from)
                    .ok()
            }))
            .build();

        let resp = self.client.send_request(requester).await?;

        let payload = resp.bytes().await?;

        let resp = serde_json::from_slice::<TencentCloudResponse>(&payload)?;

        if let Some(error_info) = resp.response.error {
            return Err(TcCloudError::TcCloud {
                code: error_info.code,
                msg: error_info.message,
            });
        }

        Ok(resp)
    }
}
