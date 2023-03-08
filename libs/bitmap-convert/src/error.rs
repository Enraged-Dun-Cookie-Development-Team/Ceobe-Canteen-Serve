
use status_err::{ErrPrefix, StatusErr};

#[derive(Debug, thiserror::Error, StatusErr)]
pub enum Error {
    #[error("数据源超出256个")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0001,
        resp_msg = "上传七牛云时出现异常"
    ))]
    LargeThen256,
    #[error("无法正确转换成Bitmap: {0}")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0001,
        resp_msg = "非正确的数据源组合id"
    ))]
    NotConvertBitmap(String),
}
