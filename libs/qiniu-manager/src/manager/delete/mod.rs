mod file_name;
use futures::StreamExt;
use qiniu_objects_manager::apis::http_client::ResponseError;
use serde_json::error;
use tracing::info;
mod delete_file;

use super::ObjectName;
use crate::{manager::delete::delete_file::DeleteIter, Error, Manager};

const BATCH_SIZE: usize = 1000;

impl Manager {
    /// 删除对象储存文件，包含三次重试
    pub async fn delete(
        &self, object_name: impl ObjectName<'_>,
    ) -> Result<(), Error> {
        info!(qiniu.bucket.delete = object_name.object_name(),);

        let bucket = &self.bucket;
        // 三次重试
        let mut res_err = Option::<ResponseError>::None;
        for _ in 0..3 {
            res_err = bucket
                .delete_object(&object_name.object_name())
                .async_call()
                .await
                .err();
            if res_err.is_none() {
                break;
            }
        }
        if let Some(err) = res_err {
            Err(err)?;
        }

        Ok(())
    }

    /// 批量删除对象存储文件
    pub async fn delete_many(
        &self, objects: Vec<impl ObjectName<'_>>,
    ) -> Result<(), Error> {
        let bucket = &self.bucket;
        let objects = objects
            .into_iter()
            .map(|obj| obj.object_name())
            .collect::<Vec<_>>();
        info!(qiniu.bucket.delete = ?objects,);

        let files_names: Vec<&str> = objects.iter().map(|s| s.as_str()).collect();

        let delete_iter = DeleteIter::<'_, '_, BATCH_SIZE> {
            files_names: &files_names,
            bucket,
        };

        let mut error: Option<_> = None;
        for mut delete in delete_iter {
            while let Some(k) = delete.next().await {
                if k.is_err() {
                    error = Some(k);
                }
            }
        }

        if let Some(result_error) = error {
            result_error?;
        }

        Ok(())
    }
}
