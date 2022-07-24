use thiserror::Error;

pub struct VideoDataSqlOperate;

#[derive(Debug, Error)]
pub enum VideoDataOperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
}

impl status_err::StatusErr for VideoDataOperateError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            VideoDataOperateError::Db(inner) => inner.prefix(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            VideoDataOperateError::Db(inner) => inner.code(),
        }
    }
}
