use rresult::{RResult, Wrap};

use crate::generate_controller;

generate_controller!(CeobeController,"/ceobe",);

/// update 获取最新的饼
/// 
/// ## Method `POST`
/// 
/// ## Input
/// ### from request head
/// - `user-auth`: 用户认证信息
/// - `device-verify`: 设备信息
/// - `last-timestamp`: 上次更新时间搓
/// ### from request body
/// N/A
/// 
async fn update()->RResult<Wrap<()>>{
    unimplemented!( )
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
async fn save_setting()->RResult<Wrap<()>>{
    unimplemented!()
}

async fn get_setting()->RResult<Wrap<()>>{
    unimplemented!()
}

