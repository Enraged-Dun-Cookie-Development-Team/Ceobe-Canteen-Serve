use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{body::HttpBody, extract::FromRef, routing::get};
use axum_starter::{
    prepare, router::Route, FromStateCollector, LoggerInitialization,
    PrepareRouteEffect, ServeAddress, ServerPrepare,
};
use ceobe_qiniu_upload::{
    BaseUrl, GetBucket, Manager, ObjectName, QiniuBaseUrl, QiniuManager,
    QiniuUpload, SecretConfig,
};
use log::SetLoggerError;
use url::Url;

#[tokio::main]
async fn main() {
    let path = std::fs::read_to_string("./qiniu_example.json")
        .expect("cannot read config");
    let config =
        serde_json::from_str::<Config>(&path).expect("Deserialize Json Fail");

    ServerPrepare::with_config(config)
        .init_logger()
        .expect("init error failure")
        .prepare_state(QiniuUpload::<_, Config>)
        .prepare_route(Router)
        .convert_state::<State>()
        .prepare_start()
        .await
        .expect("Prepare Error")
        .launch()
        .await
        .expect("Server Error");
}

#[derive(Debug, FromStateCollector, FromRef, Clone)]
struct State {
    uploader: Arc<Manager>,
    qiniu_base: QiniuBaseUrl,
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    secret: String,
    access: String,
    buckets: String,
}

impl BaseUrl for Config {
    fn get_base_url(&self) -> Url {
        "http://static.forzenstring.top/".parse().unwrap()
    }
}

impl LoggerInitialization for Config {
    type Error = SetLoggerError;

    fn init_logger(&self) -> Result<(), Self::Error> { simple_logger::init() }
}

impl ServeAddress for Config {
    type Address = SocketAddr;

    fn get_address(&self) -> Self::Address {
        SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000)
    }
}

impl GetBucket for Config {
    fn get_bucket(&self) -> &str { self.buckets.as_str() }
}

impl SecretConfig for Config {
    fn access_key(&self) -> &str { &self.access }

    fn secret_key(&self) -> &str { &self.secret }
}

impl axum_starter::ConfigureServerEffect for Config {}

#[prepare(Router)]
fn router<S, B>() -> impl PrepareRouteEffect<S, B>
where
    B: Send + Sync + 'static + HttpBody,
    S: Send + Sync + 'static + Clone,
    Arc<Manager>: FromRef<S>,
    QiniuBaseUrl: FromRef<S>,
    axum::body::Bytes: From<<B as HttpBody>::Data>,
    <B as HttpBody>::Error: std::error::Error + Send + Sync,
{
    Route::new("/api/v1/deleteMany", get(delete_combs))
}

async fn delete_combs(qiniu: QiniuManager) -> Result<&'static str, String> {
    let comb_ids: Vec<String> = (1..2401)
        .map(|i| format!("{}.json", i))
        .collect();
    
    qiniu
        .delete_many(
            comb_ids.into_iter().map(DeleteObjectName::new).collect(),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok("Delete done")
}

pub struct DeleteObjectName {
    pub file_name: String,
}

impl DeleteObjectName {
    pub fn new(name: String) -> Self { Self { file_name: name } }
}

impl<'s> ObjectName<'s> for DeleteObjectName {
    const DIR: Option<&'s str> = Some("datasource-comb");

    fn file_name(&self) -> &str { &self.file_name }
}
