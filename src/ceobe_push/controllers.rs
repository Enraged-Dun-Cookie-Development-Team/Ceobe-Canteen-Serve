use rresult::{IntoRResult, IntoRResultWithCodeError};
use std::sync::Arc;

use actix_web::{get, post, web};
use ceobe_manager::LazyLoad;
use rresult::{RResult, Wrap};

use crate::{
    ceobe_push::error::NoUpdateError, generate_controller, header_captures,
    utils::data_struct::header_info::HeaderInfo,
};

use super::error::CeobeError;

generate_controller!(CeobeController, "/ceobe", update, save_setting, get_setting);

header_captures!(pub LastUpdateTimestamp:"Last-Timestamp");
header_captures!(pub UserAuth:"User-Auth");
header_captures!(pub DeviceVerify:"Device-Verify");
header_captures!(pub FilterOut:"Filer-Out-Source");

/// update 获取最新的饼
///
/// ## Method `POST`
///
/// ## Input
/// ### from request head
/// - `user-auth`: 用户认证信息
/// - `device-verify`: 设备信息
/// - `last-timestamp`: 上次更新时间搓
/// 
/// ### from request body
/// N/A
///
#[post("/update")]
async fn update(
    updater: web::Data<Arc<ceobe_manager::UpdateLoader>>,
    last_time: HeaderInfo<LastUpdateTimestamp>,
    filter_out: HeaderInfo<FilterOut>,
) -> RResult<Wrap<LazyLoad>, CeobeError> {
    let time = last_time
        .get_one()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or_default();

    let filter = filter_out
        .iter()
        .and_then(|iter| Some(iter.collect::<Vec<_>>()))
        .unwrap_or_default();

    let res = updater
        .as_ref()
        .lazy_load(time, &filter)
        .await
        .map_err(CeobeError::from)
        .into_result()?;

    let res = res
        .into_not_empty()
        .into_result_status(NoUpdateError.into())?;

    RResult::wrap_ok(res)
}

/// 保存用户信息
///
/// ## Method `POST`
///
/// ## input
/// ### from request head
/// - `user-auth` : 用户认证信息（可选）
/// ### from request body
/// setting
///
/// ## Notice
/// 保存是如果是创建，未提供是值将会为默认值
#[post("/setting")]
async fn save_setting() -> RResult<Wrap<()>, CeobeError> {
    unimplemented!()
}

#[get("/setting")]
async fn get_setting() -> RResult<Wrap<()>, CeobeError> {
    unimplemented!()
}
