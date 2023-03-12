use tracing::info;

use crate::{Manager, error};

use super::ObjectName;

impl Manager {
    pub async fn delete(
        &self, object_name: impl ObjectName,
    ) -> Result<(), error::Error> {
        info!(
            qiniu.bucket.delete = object_name.object_name(),
        );

        let bucket = &self.bucket;
        bucket.delete_object(&object_name.object_name()).async_call().await?;
        
        Ok(())
    }
}