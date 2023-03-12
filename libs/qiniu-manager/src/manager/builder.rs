use std::time::Duration;

use qiniu_objects_manager::{ObjectsManager, Bucket};
use qiniu_upload_manager::{AutoUploader, UploadManager, UploadTokenSigner};
use qiniu_upload_token::credential::Credential;

use crate::SecretConfig;

pub struct ManagerBuilder {
    uploader: ManagedUploader,
    bucket: Bucket
}

impl ManagerBuilder {
    pub fn new(
        secret: &impl SecretConfig, name: &(impl AsRef<str> + ?Sized),
    ) -> Self {
        let credential =
            Credential::new(secret.access_key(), secret.secret_key());
        let manage = UploadManager::builder(
            UploadTokenSigner::new_credential_provider(
                credential.clone(),
                name.as_ref(),
                Duration::from_secs(3600),
            ),
        )
        .build();
        Self {
            uploader: ManagedUploader::new(manage),
            bucket: ObjectsManager::new(credential).bucket(name.as_ref()),
        }
    }

    pub fn build(self) -> super::Manager {
        crate::Manager {
            uploader: self.uploader,
            bucket: self.bucket
        }
    }
}

#[derive(Debug)]
pub struct ManagedUploader {
    default: AutoUploader,
    manage: UploadManager,
}

impl ManagedUploader {
    fn new(manage: UploadManager) -> Self {
        let default = manage.auto_uploader();
        Self { default, manage }
    }

    pub fn get_default_upload(&self) -> &AutoUploader { &self.default }

    pub fn get_manage(&self) -> &UploadManager { &self.manage }
}
