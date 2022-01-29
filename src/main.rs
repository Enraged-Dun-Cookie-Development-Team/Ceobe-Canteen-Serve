use std::net::Ipv4Addr;

use ceobe_push::dao::DataItem;
use database::{config::DbConfig, ServeDatabase};
use error::GolbalError;
use rocket::Rocket;
use tokio::{runtime, sync};
use url::Url;

mod ceobe_push;
mod database;
mod error;
mod mansion;
mod utils;

#[macro_use]
extern crate serde;

const DUN_BACKEND: &str = "ws://127.0.0.1/";
const PUSH_URL: &str = "http://localhost";
fn main()->Result<(),GolbalError> {
    // 最简单异步服务
    let rt = runtime::Builder::new_multi_thread()
        .max_blocking_threads(32)
        .enable_all()
        .build()
        .expect("Create Async Runtime Failure");

    rt.block_on(task())
}

async fn task() -> Result<(), crate::error::GolbalError> {
    let (tx,rx)=sync::mpsc::channel(16);
    let ceobe=ceobe_push::instance::Instance::new(rx);
    tokio::spawn(ceobe.run());

    Rocket::build()
    .launch().await?;

    Ok(())
}
