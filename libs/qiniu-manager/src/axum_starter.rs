use std::{convert::Infallible, ops::Deref, sync::Arc};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_starter::{prepare, state::AddState};
use futures::future::ok;
use tracing::{info, instrument};
use url::Url;

use crate::{config::BaseUrl, GetBucket, Manager, SecretConfig};

pub type QiniuUploadState = Arc<Manager>;

#[prepare(box QiniuUpload? 'c)]
#[instrument(skip(qiniu_config))]
fn init_this<'c, C>(
    qiniu_config: &'c C,
) -> Result<(AddState<Arc<Manager>>, AddState<QiniuBaseUrl>), crate::Error>
where
    C: SecretConfig + GetBucket + BaseUrl + 'static,
{
    let bucket_name = &qiniu_config.get_bucket();
    let uploader = Manager::builder(qiniu_config, bucket_name).build();

    info!(qiniu.uploader.buckets = bucket_name);
    Ok((
        AddState::new(Arc::new(uploader)),
        AddState::new(QiniuBaseUrl(Arc::new(qiniu_config.get_base_url()))),
    ))
}
#[derive(Debug, Clone)]
pub struct QiniuBaseUrl(Arc<Url>);

#[derive(Debug, Clone)]
pub struct QiniuManager {
    inner: Arc<Manager>,
    url: Arc<Url>,
}

impl QiniuManager {
    pub fn concat_url(&self, path: String) -> String {
        let mut url = self.url.deref().clone();
        url.set_path(&path);
        url.to_string()
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    #[test]
    fn test_url_concat() {
        let mut url = Url::parse("https://example.net").unwrap();
        url.set_path("a/bb");

        println!("{url}")
    }
}

impl Deref for QiniuManager {
    type Target = Manager;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<S> FromRequestParts<S> for QiniuManager
where
    S: Send + Sync,
    Arc<Manager>: FromRef<S>,
    QiniuBaseUrl: FromRef<S>,
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
        Box::pin(ok(QiniuManager {
            inner: FromRef::from_ref(state),
            url: QiniuBaseUrl::from_ref(state).0,
        }))
    }
}
