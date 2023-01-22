use std::{collections::BTreeSet, fmt::Debug, marker::Send};

use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    QuerySelect,
};
use sql_connection::{
    database_traits::get_connect::GetDatabaseConnect,
    ext_traits::{check_all_exist::QueryAllExist, CountZero},
};
use tracing::instrument;

use super::{FetcherDatasourceConfigSqlOperate, OperateResult};
use crate::fetcher::datasource_config::{
    models::model_datasource_config::{Column, Entity},
    operate::PlatformDatasource,
};

impl FetcherDatasourceConfigSqlOperate {
    /// 验证id数组是否都存在
    #[instrument(ret, skip(db))]
    pub async fn all_exist_by_id<T>(
        db: &impl ConnectionTrait, ids: T,
    ) -> OperateResult<bool>
    where
        T: IntoIterator<Item = i32> + Debug + Send,
        <T as IntoIterator>::IntoIter: Send,
    {
        let mut ids = ids.into_iter();
        let Some(first) = ids.next() else{
            return Ok(true);
        };
        // let query = gen_query_verify_all_datasource_id_exist(first, ids);
        let resp = Entity::find()
            .all_exist(
                Entity,
                Column::Id,
                first,
                ids,
                &db.get_database_backend(),
            )
            .into_model::<CountZero>()
            .one(db)
            .await?
            .unwrap()
            .take();

        Ok(resp)
    }

    /// 验证平台下是否还有数据源
    #[instrument(ret, skip(db, platforms))]
    pub async fn any_belong_to_platforms<'db, D>(
        db: &'db D, platforms: impl IntoIterator<Item = &str>,
    ) -> OperateResult<BTreeSet<String>>
    where
        D: GetDatabaseConnect + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect();
        let resp = Entity::find()
            .select_only()
            .column(Column::Platform)
            .filter(Column::Platform.is_in(platforms))
            .group_by(Column::Platform)
            .into_model::<PlatformDatasource>()
            .all(db)
            .await?;

        Ok(resp.into_iter().map(|item| item.platform).collect())
    }
}
