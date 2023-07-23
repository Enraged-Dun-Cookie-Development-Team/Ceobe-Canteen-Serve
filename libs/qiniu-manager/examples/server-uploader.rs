use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    body::HttpBody,
    extract::{FromRef, Multipart},
    routing::post,
};
use axum_starter::{
    prepare, router::Route, FromStateCollector, LoggerInitialization,
    PrepareRouteEffect, ServeAddress, ServerPrepare,
};
use ceobe_qiniu_upload::{
    BaseUrl, GetBucket, Manager, PayloadLocal, QiniuBaseUrl, QiniuManager,
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
    Route::new("/api/v1/upload", post(upload_img))
}

async fn upload_img(
    qiniu: QiniuManager, mut payload: Multipart,
) -> Result<&'static str, String> {
    // obj name
    let obj_name = payload
        .next_field()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("File info field not found")?;
    let obj_name = obj_name.text().await.map_err(|e| e.to_string())?;

    let field = payload
        .next_field()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Payload info field not found")?;
    let filename = field.file_name().map(ToOwned::to_owned);

    let local = Local { obj_name, filename };

    qiniu
        .upload_field(field, local)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Upload done")
}

struct Local {
    obj_name: String,
    filename: Option<String>,
}

impl PayloadLocal for Local {
    fn obj_name(&self) -> &str { &self.obj_name }

    fn file_name(&self) -> &str {
        self.filename.as_deref().unwrap_or(&self.obj_name)
    }
}
