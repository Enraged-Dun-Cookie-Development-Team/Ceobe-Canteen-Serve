mod axum_starter;
mod error;

mod config;
mod uploader;

pub use config::{GetBucket, SecretConfig};
pub use error::{Error, UploaderNotFound};
pub use uploader::{
    ByteUploader, FilePayload, JsonPayload, ManagedUploader, PayloadContent,
    PayloadLocal, Uploader, UploaderBuilder,ResponsePayload
};

pub use crate::axum_starter::{QiniuUpload, QiniuUploader,QiniuUploadState};
pub use mime;
pub use mime_guess;