use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{FetcherConfigSqlOperate, OperateResult};
use crate::fetcher::config::models::model_config::{self, Model};

impl FetcherConfigSqlOperate {
    #[instrument(skip(db))]
    /// 获取单个平台下的全部蹲饼器配置
    pub async fn find_single_platform_config_list<'db, D>(
        db: &'db D, platform: String,
    ) -> OperateResult<Vec<Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(fetcherConfig.platform = platform,);
        let db = db.get_connect()?;

        Ok(model_config::Entity::find()
            .filter(model_config::Column::Platform.eq(platform))
            .all(db)
            .await?)
        .tap_ok(|list| {
            Span::current().in_scope(|| {
                info!(fetcherConfig.len = list.len());
            });
        })
    }

    #[instrument(skip(db))]
    /// 获取多个平台下的全部蹲饼器配置
    pub async fn find_multi_platforms_config_list<'db, D>(
        db: &'db D, platforms: Vec<String>,
    ) -> OperateResult<Vec<Model>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            fetcherConfig.platform.list = ?platforms,
        );
        let db = db.get_connect()?;

        // 用Or拼接多个平台条件
        let mut condition = Condition::any();
        for platform in platforms {
            condition =
                condition.add(model_config::Column::Platform.eq(platform));
        }

        Ok(model_config::Entity::find()
            .filter(condition)
            .all(db)
            .await?)
        .tap_ok(|list| {
            Span::current().in_scope(|| {
                info!(fetcherConfig.len = list.len());
            });
        })
    }
}
