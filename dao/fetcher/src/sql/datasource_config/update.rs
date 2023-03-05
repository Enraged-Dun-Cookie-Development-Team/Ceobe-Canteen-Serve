use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{
        ActiveModelTrait, ConnectionTrait, IntoActiveModel,
    },
    sql_models::fetcher::datasource_config::checkers::FetcherDatasourceConfig,
};
use tracing::{info, instrument};

use super::{DatasourceOperate, OperateResult};
use crate::datasource_config::OperateError;

impl<'c, C> DatasourceOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    /// 更新数据配置到数据库
    #[instrument(ret, skip(self))]
    pub async fn update(
        &self, config: FetcherDatasourceConfig,
    ) -> OperateResult<()> {
        info!(
            config.id = config.id,
            datasource.name = config.nickname,
            datasource.avatar = config.avatar.to_string(),
            datasource.config = ?config.config
        );

        let db = self.get_connect();

        if DatasourceOperate::is_id_exist(db, config.id.unwrap()).await? {
            config.into_active_model().update(db).await?;
        }
        else {
            return Err(OperateError::DatasourceNotFound(config.id.unwrap()));
        };

        Ok(())
    }
}
