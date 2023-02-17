use sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::{info, instrument};

use super::{Datasource, OperateResult};
use crate::fetcher::datasource_config::checkers::FetcherDatasourceConfig;

impl<'c, C> Datasource<'c, C>
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
        
        if Datasource::is_id_exist(db, config.id.unwrap()).await? {
            config.into_active_model().update(db).await?;
        };

        Ok(())
    }
}
