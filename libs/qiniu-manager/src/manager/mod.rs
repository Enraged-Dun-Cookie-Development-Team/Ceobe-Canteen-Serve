mod builder;
pub mod delete;
pub mod upload;
use std::fmt::Debug;

use qiniu_objects_manager::Bucket;
pub use upload::{upload_json::JsonPayload, ResponsePayload};

pub use self::{
    builder::{ManagedUploader, ManagerBuilder},
    upload::payload::{
        ByteUploader, FilePayload, PayloadContent, PayloadLocal,
    },
};
use crate::SecretConfig;
#[derive(Debug)]
pub struct Manager {
    pub(crate) uploader: ManagedUploader,
    pub(crate) bucket: Bucket,
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
            Some(dir) => dir.to_owned() + "/" + self.file_name(),
            None => self.file_name().to_owned(),
        }
    }
}
