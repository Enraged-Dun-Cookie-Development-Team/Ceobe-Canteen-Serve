pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod verify;

use db_ops_prelude::{sea_orm::FromQueryResult, database_operates::sub_operate::{SubOperate, SuperOperate}};
use std::ops::Deref;
use abstract_database::fetcher::FetcherDatabaseOperate;
use db_ops_prelude::sea_orm;
use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;


#[derive(Debug, Error, StatusErr)]
pub enum OperateError {
    #[error("查询数据库异常: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("蹲饼器 Datasource {0} 不存在")]
    #[status_err(err(
        err_code = 0x0007,
        prefix = "ErrPrefix::NOT_FOUND",
        resp_msg = "蹲饼器数据源不存在"
    ))]
    DatasourceNotFound(i32),
    #[error("Datasources有不存在")]
    #[status_err(err(
        err_code = 0x0008,
        prefix = "ErrPrefix::NOT_FOUND",
        resp_msg = "有不存在的数据源，请刷新重新配置"
    ))]
    DatasourcesNotFound,
}

type OperateResult<T> = Result<T, OperateError>;

#[derive(FromQueryResult)]
struct PlatformDatasource {
    pub(crate) platform: String,
}


pub struct DatasourceOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for DatasourceOperate<'db, Conn> {
    type Parent = FetcherDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for DatasourceOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

pub trait ToDatasource<C> {
    fn datasource(&self) -> DatasourceOperate<'_, C>;
}

impl<C> ToDatasource<C> for FetcherDatabaseOperate<'_, C> {
    fn datasource(&self) -> DatasourceOperate<'_, C> { self.child() }
}