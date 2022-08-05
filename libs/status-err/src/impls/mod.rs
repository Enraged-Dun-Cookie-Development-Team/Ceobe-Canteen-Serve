mod mongodb;
mod sea_orm;
use std::{convert::Infallible, num::ParseIntError};

use axum::extract::rejection::{
    JsonRejection, PathRejection, QueryRejection,
};
use http::StatusCode;

use crate::{status_error, ErrPrefix};

// io prefix
status_error!(
    std::io::Error
    [
        ErrPrefix::IO,
        1:StatusCode::INTERNAL_SERVER_ERROR
    ] -> "IO时出现异常"
);
// parse prefix
status_error!(
url::ParseError[
    ErrPrefix::PARSE,
    0x0001:StatusCode::NOT_ACCEPTABLE
    ] -> "Url 解析异常"
);

status_error!(
ParseIntError[
    ErrPrefix::PARSE,
    0x0002:StatusCode::NOT_ACCEPTABLE
    ] -> "数字转换异常"
);

status_error!(
jwt::Error[
    ErrPrefix::PARSE,3
    ] -> "Jwt解析异常"
);

status_error!(
chrono::ParseError[
    ErrPrefix::PARSE,4
    ] -> "日期转换异常"
);
status_error!(
std::string::FromUtf8Error[
    ErrPrefix::PARSE,
    5:StatusCode::INTERNAL_SERVER_ERROR
    ] -> "字符串编码异常"
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
    0x00_01:StatusCode::NOT_ACCEPTABLE
    ] ->"范围检查未通过"
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
