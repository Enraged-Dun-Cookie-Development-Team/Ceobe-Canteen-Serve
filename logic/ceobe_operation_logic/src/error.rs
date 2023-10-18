use std::convert::Infallible;

use persistence::{
    ceobe_cookie::{
        analyze::OperateError as AnalyzeOperateError,
        search_content::OperateError as SearchContentOperateError,
        temp_list::OperateError as TempListOperateError,
        terra_comic::OperateError as TerraComicOperateError,
    },
    ceobe_user::property::OperateError as CeobeUserOperateError,
    fetcher::{
        datasource_combination::OperateError as DatasourceCombinationOperateError,
        datasource_config::OperateError as DatasourceOperateError,
    },
    ceobe_operate::tool_link::OperateError as ToolLinkIOperateError
};
use status_err::{ErrPrefix, HttpCode, StatusErr};
use thiserror::Error;

#[derive(Debug, Error, StatusErr)]
pub enum LogicError {
    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    ToolLinkIOperateError(#[from] ToolLinkIOperateError),
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
