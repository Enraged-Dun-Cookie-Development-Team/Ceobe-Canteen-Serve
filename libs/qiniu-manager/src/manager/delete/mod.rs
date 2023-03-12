use qiniu_objects_manager::apis::http_client::ResponseError;
use tracing::info;

use crate::{Manager, error, Error};

use super::ObjectName;

impl Manager {
    /// 删除对象储存文件，包含三次重试
    pub async fn delete(
        &self, object_name: impl ObjectName<'_>,
    ) -> Result<(), Error> {
        info!(
            qiniu.bucket.delete = object_name.object_name(),
        );

        let bucket = &self.bucket;
        // 三次重试
        let mut res_err = Option::<ResponseError>::None;
        for _ in 0..3 {
            res_err = bucket.delete_object(&object_name.object_name()).async_call().await.err();
            if res_err.is_none() {
                break;
            }
        }
        if let Some(err) = res_err {
            Err(err)?;
        }
        
        Ok(())
    }
}