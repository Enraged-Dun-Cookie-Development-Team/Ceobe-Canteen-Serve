use std::{collections::BTreeSet, fmt::Debug, marker::Send};

use db_ops_prelude::{sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect, sea_query::Func, StreamTrait,
}, database_operates::NoConnect, sql_models::fetcher::datasource_config::models::model_datasource_config::{Entity, Column}, ext_traits::{CountZero, check_all_exist::QueryAllExist, select_count::QueryCountByColumn}, get_zero_data_time, get_connect::GetDatabaseConnect, mysql_func};
use tracing::instrument;
use uuid::Uuid;

use crate::datasource_config::PlatformDatasource;

use super::{DatasourceOperate, OperateResult};

impl DatasourceOperate<'_, NoConnect> {
    /// 验证id数组是否都存在
    #[instrument(ret, skip(db))]
    pub async fn all_exist_by_id<'s, 'db, C, T>(
        db: &'db C, ids: T,
    ) -> OperateResult<bool>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
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

    #[instrument(skip(db), ret)]
    /// 是否存在该数据源，且被删除的
    pub async fn is_datasource_delete_exist<'s, 'db, C>(
        db: &'db C, datasource: &str, unique_key: &str,
    ) -> OperateResult<bool>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        Ok(Entity::find()
            .filter(Column::Datasource.eq(datasource))
            .filter(Column::DbUniqueKey.eq(unique_key))
            .filter(Column::DeleteAt.ne(get_zero_data_time()))
            .count_non_zero_by_column(Column::Id)
            .one(db)
            .await?
            .unwrap()
            .take())
    }

    #[instrument(skip(db), ret)]
    /// 是否存在该数据源，且没被删除的
    pub async fn is_id_exist<'s, 'db, C>(
        db: &'db C, did: i32,
    ) -> OperateResult<bool>
    where
        'db: 's,
        C: ConnectionTrait + StreamTrait + Send,
    {
        Ok(Entity::find()
            .filter(Column::Id.eq(did))
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .count_non_zero_by_column(Column::Id)
            .one(db)
            .await?
            .unwrap()
            .take())
    }
}

impl<'c, C> DatasourceOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait + StreamTrait,
{
    /// 验证平台下是否还有数据源
    #[instrument(ret, skip_all)]
    pub async fn any_belong_to_platforms(
        &self, platforms: impl IntoIterator<Item = &str>,
    ) -> OperateResult<BTreeSet<String>> {
        let db = self.get_connect();
        let resp = Entity::find()
            .select_only()
            .column(Column::Platform)
            .filter(Column::Platform.is_in(platforms))
            .filter(Column::DeleteAt.eq(get_zero_data_time()))
            .group_by(Column::Platform)
            .into_model::<PlatformDatasource>()
            .all(db)
            .await?;

        Ok(resp.into_iter().map(|item| item.platform).collect())
    }

    #[instrument(ret, skip_all)]
    pub async fn all_exist_by_uuid<T>(&self, uuids: T) -> OperateResult<bool>
    where
        T: IntoIterator<Item = Uuid> + Debug + Send,
        <T as IntoIterator>::IntoIter: Send,
    {
        let db = self.get_connect();

        let mut uuids = uuids.into_iter();
        let Some(first) = uuids.next() else{
            return Ok(true);
        };
        let first =
            Func::cast_as(first.hyphenated().to_string(), mysql_func::UUID);
        let uuids = uuids.map(|uuid| {
            Func::cast_as(uuid.hyphenated().to_string(), mysql_func::UUID)
        });

        let resp = Entity::find()
            .all_exist(
                Entity,
                Column::UniqueId,
                first,
                uuids,
                &db.get_database_backend(),
            )
            .one(db)
            .await?
            .unwrap()
            .take();

        Ok(resp)
    }
}
