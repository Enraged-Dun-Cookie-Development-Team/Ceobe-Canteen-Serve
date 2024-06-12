use chrono::Utc;
use secrecy::ExposeSecret;

use super::manager::TencentCloudManager;
use crate::{
    cloud_manager::{
        entities::TencentCloudResponse, signature::gen_signature,
    },
    error::TcCloudError,
    requester::TencentCloudRequester,
    task_trait::{
        serde_content::SerializeContentTrait, task_request::TaskRequestTrait,
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

        let requester = TencentCloudRequester::new(
            task,
            url,
            &authorization,
            current_time,
            payload,
        )?;

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
