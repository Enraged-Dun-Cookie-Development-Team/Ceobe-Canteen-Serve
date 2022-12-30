use std::collections::{HashMap, BTreeSet};

use sea_orm::{
    sea_query::Cond, ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr,
    EntityTrait, QueryFilter, QuerySelect, Statement,
};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tracing::instrument;

use super::{
    DatasourceCounts, FetcherDatasourceConfigSqlOperate, OperateResult,
};
use crate::fetcher::datasource_config::{
    models::model_datasource_config, operate::PlatformDatasourceCounts,
};

impl FetcherDatasourceConfigSqlOperate {
    // 验证id数组是否都存在
    #[instrument(ret, skip(db))]
    pub async fn has_all_datasource_ids<'db, D>(
        db: &'db D, ids: Vec<i32>,
    ) -> OperateResult<bool>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let mut sql = String::from("select count(B.id) from (");
        for id in ids {
            sql.push_str(&format!(r#" select {id} as id from dual union"#));
        }
        sql = sql.trim_end_matches("union").to_string();
        sql.push_str(
            ") B left join fetcher_datasource_config on \
             fetcher_datasource_config.id = B.id where \
             fetcher_datasource_config.id is null;",
        );

        let db = db.get_connect()?;
        let resp = model_datasource_config::Entity::find()
            .from_raw_sql(Statement::from_string(DatabaseBackend::MySql, sql))
            .into_model::<DatasourceCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count == 0)
    }

    // 验证平台下是否还有数据源
    #[instrument(ret, skip(db))]
    pub async fn has_datasource_from_platforms<'db, D>(
        db: &'db D, platforms: Vec<String>,
    ) -> OperateResult<BTreeSet<String>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let mut condition = Cond::any();
        for platform in platforms {
            condition = condition
                .add(model_datasource_config::Column::Platform.eq(platform))
        }

        let db = db.get_connect()?;
        let resp = model_datasource_config::Entity::find()
            .select_only()
            .column_as(model_datasource_config::Column::Id.count(), "count")
            .column(model_datasource_config::Column::Platform)
            .filter(condition)
            .group_by(model_datasource_config::Column::Platform)
            .into_model::<PlatformDatasourceCounts>()
            .all(db)
            .await?;

        let mut exist_map = BTreeSet::new();
        for platform_datasource in resp {
            exist_map.insert(platform_datasource.platform);
        }

        Ok(exist_map)
    }
}
