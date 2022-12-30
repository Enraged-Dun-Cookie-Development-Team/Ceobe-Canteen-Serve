mod mongodb;
mod sea_orm;
use std::{convert::Infallible, num::ParseIntError};

use axum::extract::rejection::{
    JsonRejection, PathRejection, QueryRejection,
};
use checker::prefabs::num_check::NonZeroUnsignedError;
use http::StatusCode;

use crate::{status_error, ErrPrefix, StatusErr};

// io prefix
status_error!(
    std::io::Error
    [
        ErrPrefix::IO,
        0x0001:StatusCode::INTERNAL_SERVER_ERROR
    ] -> "IO时出现异常"
);
// parse prefix
status_error!(
url::ParseError[
    ErrPrefix::PARSE,
    0x0001
    ] -> "Url 解析异常"
);

status_error!(
ParseIntError[
    ErrPrefix::PARSE,
    0x0002
    ] -> "数字转换异常"
);

status_error!(
jwt::Error[
    ErrPrefix::PARSE, 0x0003
    ] -> "Jwt解析异常"
);

status_error!(
chrono::ParseError[
    ErrPrefix::PARSE, 0x0004
    ] -> "日期转换异常"
);
status_error!(
std::string::FromUtf8Error[
    ErrPrefix::PARSE,
    0x0005:StatusCode::INTERNAL_SERVER_ERROR
    ] -> "字符串编码异常"
);

status_error!(
    http::header::ToStrError[
        ErrPrefix::PARSE,
        0x0006
    ] -> "http 请求头内容解析异常"
);
status_error!(
    http::header::InvalidHeaderValue[
        ErrPrefix::PARSE,
        0x0007: StatusCode::INTERNAL_SERVER_ERROR
    ] -> "非法 Http 请求头内容"
);

// check prefix
status_error!(
Infallible[
    ErrPrefix::CHECKER,
    0x00_00
    ]->""
);
status_error!(
    range_limit::Error[
    ErrPrefix::CHECKER,
    0x00_01
    ] ->"范围检查未通过"
);
use serde_json::Error as JsonError;

status_error!(
JsonError[
    ErrPrefix::CHECKER,
    0x00_04
    ]->"`Json`请求解析异常"
);

status_error!(
JsonRejection[
    ErrPrefix::CHECKER,
    0x00_04
    ]->"`Json`请求解析异常"
);
status_error!(
PathRejection[
    ErrPrefix::CHECKER,
    0x00_05
    ]-> "`Path`数据加载异常"
);
status_error!(
QueryRejection[
    ErrPrefix::CHECKER,
    0x00_07
    ] -> "请求的`Query`解析失败"
);
status_error!(
bincode::Error[
    ErrPrefix::CHECKER,
    0x00_0C
    ] -> "`Bincode` 序列化/反序列化异常 "
);
use axum::extract::multipart::{MultipartError, MultipartRejection};
status_error!(
    MultipartRejection[
        ErrPrefix::CHECKER,
        0x000F
    ] -> "获取`MultiPart`异常 "
);

status_error!(
    MultipartError[
        ErrPrefix::CHECKER,
        0x000F
    ] -> "解析`MultiPart`异常 "
);

// authorized prefix
status_error!(
    bcrypt::BcryptError[
        ErrPrefix::UNAUTHORIZED,
        5:StatusCode::INTERNAL_SERVER_ERROR
    ] -> "密码校验异常"
);

status_error!(
    reqwest::Error[
        ErrPrefix::NOT_FOUND,
        3 : StatusCode::INTERNAL_SERVER_ERROR
    ] -> "发起请求时异常"
);

status_error!(
    NonZeroUnsignedError[
        ErrPrefix::CHECKER,
        0x00_0E
    ] -> "预期为0值取得非0值"
);

use checker::prefabs::{
    json_obj_check::JsonObjError, no_remainder_checker::HasRem,
};

impl<const RHS: u64> StatusErr for HasRem<RHS> {
    fn prefix(&self) -> ErrPrefix { ErrPrefix::CHECKER }

    fn code(&self) -> u16 { 0x0014 }
}

status_error!(
    JsonObjError[
        ErrPrefix::CHECKER,
        0x0014
    ]->"Json 对象不符合预期"
);
