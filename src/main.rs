use std::collections::HashMap;

use ceobe_push::{
    dao::{DataItem, DataSource},
    instance::DataCollect,
};

use error::GolbalError;
use rocket::{get, Rocket, State};
use rresult::{RResult, Wrap};
use tokio::{
    runtime,
    sync::watch::{self, Ref},
};
use url::Url;

mod ceobe_push;
mod database;
mod error;
mod mansion;
mod utils;

#[macro_use]
extern crate serde;

fn main() -> Result<(), GolbalError> {
    // 最简单异步服务
    let rt = runtime::Builder::new_multi_thread()
        .max_blocking_threads(32)
        .enable_all()
        .build()
        .expect("Create Async Runtime Failure");

    rt.block_on(task())
}

async fn task() -> Result<(), crate::error::GolbalError> {
    // 启动 ws客户端
    let (ceobe, updater) = ceobe_push::instance::Instance::new();
    tokio::spawn(ceobe.run());
    let recv = updater.run();
    // 启动rocket 客户端
    Rocket::build()
        .manage(recv)
        .mount("/", rocket::routes![handle, abab])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
async fn handle(
    msgr: &State<watch::Receiver<HashMap<DataSource, DataCollect>>>,
) -> RResult<Ref<'_, HashMap<DataSource, DataCollect>>, GolbalError> {
    let w = msgr.inner();
    let v = w.borrow();

    RResult::ok(v)
}
#[get("/i")]
async fn abab() -> RResult<Wrap<String>, GolbalError> {
    RResult::wrap_ok("ABAB".to_string())
}
