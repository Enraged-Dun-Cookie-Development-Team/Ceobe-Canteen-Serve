use rresult::{IntoRResult, IntoRResultWithCodeError};

use actix_web::{get, post, web};
use ceobe_manager::LazyLoad;
use rresult::{RResult, Wrap};

use crate::{
    ceobe_push::{error::NoUpdateError, model::DataSourceFilter},
    generate_controller,
    utils::req_pretreatment::{
        prefabs::{Json, MapErr, ToRResult},
        ReqPretreatment,
    },
};

use super::error::CeobeError;

generate_controller!(CeobeController, "/ceobe", update, save_setting, get_setting);

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
    updater: web::Data<ceobe_manager::UpdateLoader>,
    filter: ReqPretreatment<ToRResult<MapErr<Json<DataSourceFilter>, CeobeError>>>,
) -> RResult<Wrap<LazyLoad>, CeobeError> {
    let filter = filter.unwrap()?;
    let res = updater
        .as_ref()
        .lazy_load(&filter)
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
#[post("/setting/{id}")]
async fn save_setting() -> RResult<Wrap<()>, CeobeError> {
    unimplemented!()
}

#[get("/setting")]
async fn get_setting() -> RResult<Wrap<()>, CeobeError> {
    unimplemented!()
}
