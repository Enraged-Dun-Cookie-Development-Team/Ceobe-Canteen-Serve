use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{
        ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
        StreamTrait,
    },
    sql_models::fetcher::config::models::model_config::{self, Model},
};
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{ConfigOperate, OperateResult};

impl<'c, C> ConfigOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    #[instrument(skip(self))]
    /// 获取单个平台下的全部蹲饼器配置
    pub async fn find_all_by_platform(
        &self, platform: &str,
    ) -> OperateResult<Vec<Model>> {
        info!(fetcherConfig.platform = platform,);
        let db = self.get_connect();

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

    #[instrument(skip(self))]
    /// 获取多个平台下的全部蹲饼器配置
    pub async fn find_multi_platforms_config_list(
        &self, platforms: Vec<String>,
    ) -> OperateResult<Vec<Model>> {
        info!(
            fetcherConfig.platform.list = ?platforms,
        );
        let db = self.get_connect();

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
