use std::{convert::Infallible, ops::Deref, sync::Arc};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_starter::{prepare, state::AddState};
use futures::future::ok;
use tracing::{info, instrument};

use crate::{GetBucket, SecretConfig, Uploader};

pub type QiniuUploadState = Arc<Uploader>;

#[prepare(box QiniuUpload? 'c)]
#[instrument(skip(qiniu_config))]
fn init_this<'c, C>(
    qiniu_config: &'c C,
) -> Result<AddState<Arc<Uploader>>, crate::Error>
where
    C: SecretConfig + GetBucket + 'static,
{
    let bucket_name = &qiniu_config.get_bucket();
    let uploader = Uploader::builder(qiniu_config, bucket_name).build();

    info!(qiniu.uploader.buckets = bucket_name);
    Ok(AddState::new(Arc::new(uploader)))
}

pub struct QiniuUploader {
    inner: Arc<Uploader>,
}

impl Deref for QiniuUploader {
    type Target = Uploader;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<S> FromRequestParts<S> for QiniuUploader
where
    S: Send + Sync,
    Arc<Uploader>: FromRef<S>,
{
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _parts: &'life0 mut Parts, state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(ok(QiniuUploader {
            inner: FromRef::from_ref(state),
        }))
    }
}
