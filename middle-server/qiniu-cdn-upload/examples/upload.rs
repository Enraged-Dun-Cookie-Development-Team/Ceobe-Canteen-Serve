use std::marker::PhantomData;

use axum::{
    body::Body,
    extract::{FromRef, Multipart},
    http::StatusCode,
    response::Html,
    routing::post,
};
use axum_starter::{
    prepare,
    router::{Fallback, Route},
    Configure, FromStateCollector, PrepareRouteEffect, Provider,
    ServerPrepare,
};
use ceobe_qiniu_upload::{
    GetBucket, QiniuUpload, QiniuUploadState, QiniuUploader, ResponsePayload,
    SecretConfig,
};

use futures::FutureExt;
use qiniu_cdn_upload::{
    update_payload::UploadPayload,
    update_source::{FieldSource, UploadSource},
};

async fn upload(
    uploader: QiniuUploader, mut file: Multipart,
) -> Result<axum::Json<ResponsePayload>, String> {
    let source = file
        .next_field()
        .await
        .map_err(|err| format!("load field error {:?}", err))?
        .ok_or("No field".to_string())?;

    let v = qiniu_cdn_upload::upload(
        &uploader,
        source,
        ImagePayload::<FieldSource>::new(),
    )
    .await
    .map_err(|err| format!("upload error {:?}", err))?;

    Ok(axum::Json(v))
}

struct ImagePayload<Source>(PhantomData<Source>, String);

impl<Source> ImagePayload<Source> {
    fn new() -> Self {
        Self(PhantomData, uuid::Uuid::new_v4().to_string())
    }
}

impl<Source> UploadPayload for ImagePayload<Source>
where
    Source: UploadSource + 'static,
{
    type Source = Source;

    const DIR: &'static str = "image";

    fn obj_name(&self) -> &str {
        self.1.as_str()
    }
}

#[prepare(Router)]
fn set_route() -> impl PrepareRouteEffect<State, Body> {
    Route::new("/upload", post(upload))
}

#[prepare(Fallback)]
fn fall_back() -> impl PrepareRouteEffect<State, Body> {
    Fallback::new(|| async {
        (
            StatusCode::BAD_REQUEST,
            Html(r#"请前往 <a herf = "/upload">/upload</a>"#),
        )
    })
}

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("异步运行时启动异常");

    rt.block_on(server())
}

async fn server() {
    ServerPrepare::with_config(Config::default())
        .init_logger()
        .expect("初始化日志异常")
        .prepare_state(QiniuUpload::<_, QiniuConfig>)
        .prepare_route(Router)
        .prepare_route(Fallback)
        .graceful_shutdown(ctrl_c().map(|_| ()))
        .convert_state()
        .prepare_start()
        .await
        .expect("准备阶段异常")
        .launch()
        .await
        .expect("服务器异常");
}

use log::Level::Debug;
use log::SetLoggerError;
use simple_logger::init_with_level;
use std::net::Ipv4Addr;
use tokio::{runtime, signal::ctrl_c};

#[derive(Debug, Default, Provider, Configure)]
#[conf(
    address(func(
        path = "||(Ipv4Addr::LOCALHOST,8080)",
        associate,
        ty = "(Ipv4Addr, u16)"
    )),
    logger(
        func = "||init_with_level(Debug)",
        associate,
        error = "SetLoggerError"
    )
)]
struct Config {
    #[provider(ref, transparent)]
    qiniu: QiniuConfig,
}

#[derive(Debug, Clone, FromStateCollector, FromRef)]
pub struct State {
    uploader: QiniuUploadState,
}

#[derive(Debug)]
pub struct QiniuConfig {
    secret: String,
    access: String,
    bucket: String,
}

impl GetBucket for QiniuConfig {
    fn get_bucket(&self) -> &str {
        &self.bucket
    }
}

impl SecretConfig for QiniuConfig {
    fn access_key(&self) -> &str {
        &self.access
    }

    fn secret_key(&self) -> &str {
        &self.secret
    }
}

//impl `Default` and associate function `new` for
// [QiniuConfig]
include!("../../../qiniu_info.meta");
