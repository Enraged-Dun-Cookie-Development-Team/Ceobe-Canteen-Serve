mod axum_starter;
mod error;

mod config;
mod manager;

pub use config::{BaseUrl, GetBucket, SecretConfig};
pub use error::{Error, UploaderNotFound};
pub use mime;
pub use mime_guess;
pub use manager::{
    ByteUploader, FilePayload, JsonPayload, ManagedUploader, PayloadContent,
    PayloadLocal, ResponsePayload, Manager, ManagerBuilder,
};

pub use crate::axum_starter::{
    QiniuBaseUrl, QiniuUpload, QiniuUploadState, QiniuManager,
};
