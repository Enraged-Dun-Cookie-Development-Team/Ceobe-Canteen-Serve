mod axum_starter;
mod error;

mod config;
mod uploader;

pub use config::SecretConfig;
pub use error::{Error, UploaderNotFound};
pub use uploader::{
    FilePayload, JsonPayload, ManagedUploader, PayloadContent, PayloadLocal, Uploader,
    UploaderBuilder,
};

pub use crate::axum_starter::{QiniuUpload, QiniuUploader};
