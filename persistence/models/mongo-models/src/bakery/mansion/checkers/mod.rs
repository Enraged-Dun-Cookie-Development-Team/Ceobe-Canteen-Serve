use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

pub mod daily;
pub mod each_info;
pub mod id_checker;
pub mod mansion;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("数据长度校验异常: {0}")]
    Size(#[from] range_limit::Error),
    #[error("未知饼学大厦ID格式: {0:?}")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x_00_02,
        http_code = "HttpCode::BAD_REQUEST"
    ))]
    UnknownMansionIdFormat(String),
    #[error("未知的预期确信度等级: {0:?}")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x_00_06,
        http_code = "HttpCode::BAD_REQUEST"
    ))]
    UnknownPredictType(String),
    #[error("错误的Fraction值: [{0}] 范围(0~5)")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x_00_03,
        http_code = "HttpCode::BAD_REQUEST"
    ))]
    BadFraction(i16),
    #[error("饼学大厦日期格式异常: {0}")]
    MansionDataFormat(#[from] chrono::ParseError),
}
