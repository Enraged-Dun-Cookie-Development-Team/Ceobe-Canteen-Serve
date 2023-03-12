pub mod delete;
pub mod upload;
mod builder;
use std::fmt::Debug;

use futures::Future;
use qiniu_objects_manager::{ObjectsManager, Bucket};
use qiniu_upload_manager::AutoUploaderObjectParams;
use tracing::info;
pub use upload::upload_json::JsonPayload;
pub use upload::ResponsePayload;

pub use self::{
    builder::{ManagedUploader, ManagerBuilder},
    upload::payload::{ByteUploader, FilePayload, PayloadContent, PayloadLocal},
};
use crate::{error, SecretConfig};
#[derive(Debug)]
pub struct Manager {
    pub(crate) uploader: ManagedUploader,
    pub(crate) bucket: Bucket
}

impl Manager {
    pub fn builder(
        secret: &impl SecretConfig, name: &(impl AsRef<str> + ?Sized),
    ) -> builder::ManagerBuilder {
        ManagerBuilder::new(secret, name)
    }
}

pub trait ObjectName<'s> {
    const DIR: Option<&'s str>;

    fn file_name(&self) -> &str;
    
    fn object_name(&self) -> String {
        match Self::DIR {
            Some(dir) => dir.to_owned() +"/"+ self.file_name(),
            None => self.file_name().to_owned()
        }
    }
}
