use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, IntoActiveModel};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};
use crate::fetcher::datasource_config::checkers::FetcherDatasourceConfig;

impl FetcherDatasourceConfigSqlOperate {
    /// 更新数据配置到数据库
    #[instrument(ret, skip(db))]
    pub async fn update<'db, D>(
        db: &'db D, config: FetcherDatasourceConfig,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            config.id = config.id,
            datasource.name = config.nickname,
            datasource.avatar = config.avatar.to_string(),
            datasource.config = ?config.config
        );

        let db = db.get_connect()?;

        config.into_active_model().update(db).await?;

        Ok(())
    }
}
