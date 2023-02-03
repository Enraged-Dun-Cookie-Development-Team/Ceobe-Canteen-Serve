mod axum_starter;
mod error;

mod config;
mod uploader;

pub use config::{BaseUrl, GetBucket, SecretConfig};
pub use error::{Error, UploaderNotFound};
pub use mime;
pub use mime_guess;
pub use uploader::{
    ByteUploader, FilePayload, JsonPayload, ManagedUploader, PayloadContent,
    PayloadLocal, ResponsePayload, Uploader, UploaderBuilder,
};

pub use crate::axum_starter::{
    QiniuBaseUrl, QiniuUpload, QiniuUploadState, QiniuUploader,
};
