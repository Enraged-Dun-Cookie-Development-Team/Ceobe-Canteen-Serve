use std::net::{Ipv4Addr, SocketAddr};

use axum::{extract::Multipart, routing::post};
use axum_starter::{
    prepare, router::Route, LoggerInitialization, PreparedEffect,
    ServeAddress, ServerPrepare,
};
use ceobe_qiniu_upload::{
    GetBucket, PayloadLocal, QiniuUpload, QiniuUploader, SecretConfig,
};
use log::SetLoggerError;
#[tokio::main]
async fn main() {
    let path = std::fs::read_to_string("./qiniu_example.json")
        .expect("cannot read config");
    let config =
        serde_json::from_str::<Config>(&path).expect("Deserialize Json Fail");

    ServerPrepare::with_config(config)
        .init_logger()
        .expect("init error failure")
        .append(QiniuUpload::<_, Config>)
        .append(Router)
        .prepare_start()
        .await
        .expect("Prepare Error")
        .launch()
        .await
        .expect("Server Error");
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    secret: String,
    access: String,
    buckets: Vec<String>,
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
    type BucketName = String;
    type Iterator<'i> = std::slice::Iter<'i, String>
    where
        Self: 'i;

    fn get_buckets(&self) -> Self::Iterator<'_> { self.buckets.iter() }
}

impl SecretConfig for Config {
    fn access_key(&self) -> &str { &self.access }

    fn secret_key(&self) -> &str { &self.secret }
}

impl axum_starter::ConfigureServerEffect for Config {}

#[prepare(Router)]
fn router() -> impl PreparedEffect {
    Route::new("/api/v1/upload", post(upload_img))
}

async fn upload_img(
    qiniu: QiniuUploader, mut payload: Multipart,
) -> Result<&'static str, String> {
    // obj name
    let obj_name = payload
        .next_field()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("File info field not found")?;
    let obj_name = obj_name.text().await.map_err(|e| e.to_string())?;
    // bucket name
    let bucket = payload
        .next_field()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Bucket info field not found")?;
    let bucket = bucket.text().await.map_err(|e| e.to_string())?;

    let field = payload
        .next_field()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Payload info field not found")?;
    let filename = field.file_name().map(ToOwned::to_owned);

    let local = Local {
        obj_name,
        bucket,
        filename,
    };

    qiniu
        .upload_field(field, local)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Upload done")
}

struct Local {
    obj_name: String,
    bucket: String,
    filename: Option<String>,
}

impl PayloadLocal for Local {
    fn bucket(&self) -> &str { &self.bucket }

    fn obj_name(&self) -> &str { &self.obj_name }

    fn file_name(&self) -> &str {
        self.filename.as_deref().unwrap_or(&self.obj_name)
    }
}
