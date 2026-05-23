use status_err::{
    generated_error::{
        checker_kind::LargeThen256Error, parse_kind::NotConvertBitmapError,
    },
    StatusErr,
};

#[derive(Debug, thiserror::Error, StatusErr)]
pub enum Error {
    #[error("数据源超出256个")]
    #[status_err(err(bind = "LargeThen256Error"))]
    LargeThen256,
    #[error("无法正确转换成Bitmap: {0}")]
    #[status_err(err(bind = "NotConvertBitmapError"))]
    NotConvertBitmap(String),
}
