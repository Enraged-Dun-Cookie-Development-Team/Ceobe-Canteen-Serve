use general_request_client::HeaderValue;
use secrecy::ExposeSecret;
use serde::Serialize;

use super::cloud_manager::TencentCloudManager;
use crate::{
    cloud_manager::{
        entities::{CommonParameter, RequestContent, TencentCloudResponse},
        signature::gen_signature,
    },
    error::TcCloudError,
    requester::TencentCloudRequester,
    task_trait::{
        header_fetch::{ContentType, HeaderFetch, Host},
        serde_content::SerializeContentTrait,
        task_request::TaskRequestTrait,
    },
};

impl TencentCloudManager {
    /// 通用请求
    pub(crate) async fn common_request<Task>(
        &self, task: &Task,
    ) -> Result<TencentCloudResponse, TcCloudError>
    where
        TcCloudError: From<<Task::Payload as SerializeContentTrait>::Error>,
        Task: TaskRequestTrait,
    {
        let url = Task::SERVICE.to_url()?;
        let payload = task.payload().serialize_to()?;

        let authorization = gen_signature(
            self.id.expose_secret(),
            self.key.expose_secret(),
            task,
            &url,
            &payload,
        )?;

        let requester = TencentCloudRequester::<Task>::builder()
            .url(url.clone())
            .payload(payload)
            .task(task)
            .host(Host.fetch_header(task, &url)?)
            .action(HeaderValue::from_str(Task::ACTION)?)
            .version(Task::VERSION.header_value())
            .timestamp(HeaderValue::from_str(&task.timestamp().to_string())?)
            .content_type(ContentType.fetch_header(task, &url)?)
            .authorization(HeaderValue::from_str(&authorization)?)
            .region(
                Task::REGION
                    .map(|region| HeaderValue::from_str(&region))
                    .transpose()?,
            )
            .token(
                Task::TOKEN
                    .map(|token| HeaderValue::from_str(&token))
                    .transpose()?,
            )
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
