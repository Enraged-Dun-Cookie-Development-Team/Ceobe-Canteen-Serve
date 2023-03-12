mod axum_starter;
mod error;

mod config;
mod manager;

pub use config::{BaseUrl, GetBucket, SecretConfig};
pub use error::{Error, UploaderNotFound};
pub use manager::{
    ByteUploader, FilePayload, JsonPayload, ManagedUploader, Manager,
    ManagerBuilder, ObjectName, PayloadContent, PayloadLocal,
    ResponsePayload,
};
pub use mime;
pub use mime_guess;

pub use crate::axum_starter::{
    QiniuBaseUrl, QiniuManager, QiniuUpload, QiniuUploadState,
};
