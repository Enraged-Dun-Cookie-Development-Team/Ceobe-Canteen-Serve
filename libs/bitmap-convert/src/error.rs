use status_err::{ErrPrefix, StatusErr};

#[derive(Debug, thiserror::Error, StatusErr)]
pub enum Error {
    #[error("数据源超出256个")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001A,
        resp_msg = "数据源超出上限，请联系管理员"
    ))]
    LargeThen256,
    #[error("无法正确转换成Bitmap: {0}")]
    #[status_err(err(
        prefix = "ErrPrefix::PARSE",
        err_code = 0x0008,
        resp_msg = "非正确的数据源组合id"
    ))]
    NotConvertBitmap(String),
}
