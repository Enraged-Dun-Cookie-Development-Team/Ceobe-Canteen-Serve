use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

pub mod daily;
pub mod each_info;
pub mod id_checker;
pub mod mansion;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, Error)]
pub enum MansionDataCheckerError {
    #[error("数据长度校验异常: {0}")]
    Size(#[from] range_limit::Error),
    #[error("未知饼学大厦ID格式: {0:?}")]
    UnknownMansionIdFormat(String),
    #[error("未知的预期确信度等级: {0:?}")]
    UnknownPredictType(String),
    #[error("错误的Fraction值: [{0}] 范围(0~5)")]
    BadFraction(i16),
    #[error("饼学大厦日期格式异常: {0}")]
    MansionDataFormat(#[from] chrono::ParseError),
}

impl StatusErr for MansionDataCheckerError {
    fn prefix(&self) -> status_err::ErrPrefix {
        use MansionDataCheckerError::*;
        match self {
            Size(rl) => rl.prefix(),
            MansionDataFormat(_) => ErrPrefix::PARSE,
            UnknownPredictType(_)
            | BadFraction(_)
            | UnknownMansionIdFormat(_) => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        use MansionDataCheckerError::*;
        match self {
            Size(rl) => rl.code(),
            UnknownMansionIdFormat(_) => 0x_00_02,
            UnknownPredictType(_) => 0x_00_06,
            BadFraction(_) => 0x_00_03,
            MansionDataFormat(_) => 0x_00_04,
        }
    }

    fn http_code(&self) -> status_err::HttpCode {
        use MansionDataCheckerError::*;
        match self {
            Size(rl) => rl.http_code(),
            MansionDataFormat(_)
            | UnknownMansionIdFormat(_)
            | UnknownPredictType(_)
            | BadFraction(_) => HttpCode::NOT_ACCEPTABLE,
        }
    }
}
