use futures::StreamExt;
use qiniu_objects_manager::apis::http_client::ResponseError;
use tracing::info;

use super::ObjectName;
use crate::{Error, Manager};

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
        
        let mut v = objects
            .iter()
            .fold(bucket.batch_ops(), |mut ops, obj| {
                ops.add_operation(bucket.delete_object(obj.as_str()));
                ops
            })
            .async_call();

        while let Some(k) = v.next().await {
            k?;
        }

       Ok(())
    }
}
