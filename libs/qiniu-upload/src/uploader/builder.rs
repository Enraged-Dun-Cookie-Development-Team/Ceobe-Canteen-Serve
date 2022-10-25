use std::{collections::HashMap, time::Duration};

use qiniu_upload_manager::{AutoUploader, UploadManager, UploadTokenSigner};
use qiniu_upload_token::credential::Credential;
use smallstr::SmallString;

use crate::{error, SecretConfig};

pub struct UploaderBuilder {
    credential: Credential,
    managers:
        HashMap<SmallString<[u8; 64]>, ManagedUploader, ahash::RandomState>,
}

impl UploaderBuilder {
    pub fn new(secret: &impl SecretConfig) -> Self {
        Self {
            credential: Credential::new(
                secret.access_key(),
                secret.secret_key(),
            ),
            managers: HashMap::with_hasher(ahash::RandomState::new()),
        }
    }

    pub fn add_bucket(
        mut self, name: &(impl AsRef<str> + ?Sized),
    ) -> Result<Self, error::Error> {
        if !self.managers.contains_key(name.as_ref()) {
            let manage = UploadManager::builder(
                UploadTokenSigner::new_credential_provider(
                    self.credential.clone(),
                    name.as_ref(),
                    Duration::from_secs(3600),
                ),
            )
            .build();

            self.managers
                .insert(name.as_ref().into(), ManagedUploader::new(manage));
        }
        Ok(self)
    }

    pub fn build(self) -> super::Uploader {
        crate::Uploader {
            managers: self.managers,
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
