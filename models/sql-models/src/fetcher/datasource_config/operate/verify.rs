use std::{collections::BTreeSet, fmt::Debug, marker::Send};

use mysql_func::UuidToBin;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
};
use sea_query::Func;
use sql_connection::{
    database_traits::{
        database_operates::NoConnect, get_connect::GetDatabaseConnect,
    },
    ext_traits::{check_all_exist::QueryAllExist, CountZero},
};
use tracing::instrument;
use uuid::Uuid;

use super::{Datasource, OperateResult};
use crate::fetcher::datasource_config::{
    models::model_datasource_config::{Column, Entity},
    operate::PlatformDatasource,
};

impl Datasource<'_, NoConnect> {
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
}
impl<'c, C> Datasource<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
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
            .group_by(Column::Platform)
            .into_model::<PlatformDatasource>()
            .all(db)
            .await?;

        Ok(resp.into_iter().map(|item| item.platform).collect())
    }

    #[instrument(ret, skip_all)]
    pub async fn all_exist_by_uuid<T>(
        &self, uuids: T,
    ) -> OperateResult<bool> 
    where
        T: IntoIterator<Item = Uuid> + Debug + Send,
        <T as IntoIterator>::IntoIter: Send,
    {
        let db = self.get_connect();
        
        let mut uuids = uuids.into_iter();
        let Some(first) = uuids.next() else{
            return Ok(true);
        };
        let first = Func::cust(UuidToBin).arg(first.hyphenated().to_string());
        let uuids = uuids.map(|uuid| Func::cust(UuidToBin).arg(uuid.hyphenated().to_string()));
        
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
