use std::{convert::Infallible, ops::Deref, sync::Arc};

use axum_core::extract::FromRequest;
use axum_starter::{extension::SetExtension, prepare, PreparedEffect};
use futures::future::ok;

use crate::{SecretConfig, Uploader, UploaderNotFound};

#[prepare(box QiniuUpload 'c)]
fn init_this<'c, C, I>(
    secret: &'c C,
    exist_buckets: I,
) -> Result<impl PreparedEffect, crate::Error>
where
    C: SecretConfig,
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let uploader = Uploader::builder(secret);

    let uploader = exist_buckets
        .into_iter()
        .try_fold(uploader, |uploader, bucket_name| {
            uploader.add_bucket(bucket_name.as_ref())
        })?
        .build();

    Ok(SetExtension::arc(uploader))
}

pub struct QiniuUploader {
    inner: Arc<Uploader>,
}

impl Deref for QiniuUploader {
    type Target = Uploader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<B> FromRequest<B> for QiniuUploader {
    type Rejection = Infallible;

    fn from_request<'life0, 'async_trait>(
        req: &'life0 mut axum_core::extract::RequestParts<B>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(ok(QiniuUploader {
            inner: req
                .extensions()
                .get::<Arc<Uploader>>()
                .ok_or(UploaderNotFound)
                .expect("Uploader Not found")
                .to_owned(),
        }))
    }
}
