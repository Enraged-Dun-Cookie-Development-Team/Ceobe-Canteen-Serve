use chrono::Utc;
use general_request_client::HeaderValue;
use secrecy::ExposeSecret;

use super::manager::TencentCloudManager;
use crate::{
    cloud_manager::{
        entities::TencentCloudResponse, signature::gen_signature,
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
    pub async fn exec_request<'r, Task>(
        &self, task: &'r Task,
    ) -> Result<TencentCloudResponse, TcCloudError>
    where
        TcCloudError:
            From<<Task::Payload<'r> as SerializeContentTrait>::Error>,
        Task: TaskRequestTrait + 'r,
    {
        let url = Task::SERVICE.to_url()?;
        let payload = task.payload().serialize_to()?;
        let current_time = Utc::now();

        let authorization = gen_signature(
            self.id.expose_secret(),
            self.key.expose_secret(),
            task,
            &url,
            &payload,
            &current_time,
        )?;

        let requester = TencentCloudRequester::<Task>::builder()
            .payload(payload)
            .task(task)
            .host(Host.fetch_header(task, &url)?)
            .action(HeaderValue::from_str(Task::ACTION)?)
            .version(Task::VERSION.header_value())
            .timestamp(HeaderValue::from_str(
                &current_time.timestamp().to_string(),
            )?)
            .content_type(ContentType.fetch_header(task, &url)?)
            .authorization(HeaderValue::from_str(&authorization)?)
            .region(Task::REGION.map(HeaderValue::from_str).transpose()?)
            .token(Task::TOKEN.map(HeaderValue::from_str).transpose()?)
            .url(url)
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
