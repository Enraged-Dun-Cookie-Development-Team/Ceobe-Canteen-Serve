use std::collections::HashSet;

use abstract_database::{ceobe::ToCeobe, fetcher::ToFetcher};
use ceobe_user::ToCeobeUser;
use checker::LiteChecker;
use db_ops_prelude::{
    bool_or::TrueOrError,
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::user_property::{
        check::user_checker::{UserPropertyChecker, UserPropertyUncheck},
        models::{UserPropertyChecked, UserMobId},
    },
    mongodb::bson,
    SqlDatabaseOperate,
};
use fetcher::datasource_config::{OperateError as FetcherDatasourceOperateError, ToDatasource};
use futures::future;
use tokio::task;
use tracing::warn;
use uuid::Uuid;
use uuids_convert::{vec_bson_uuid_to_uuid, vec_uuid_to_bson_uuid};

use crate::{
    error,
    error::LogicResult,
    view::{DatasourceConfig, MobIdReq},
};

pub struct CeobeUserLogic;

impl CeobeUserLogic {
    /// 新建数据源配置
    pub async fn create_user(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate, mob_id: MobIdReq,
    ) -> LogicResult<()> {
        // TODO: 验证mob_id是否为小刻食堂旗下mob id

        // 获取所有数据源的uuid列表
        let datasource_uuids = db
            .fetcher()
            .datasource()
            .find_all_uuid()
            .await?
            .into_iter()
            .map(|uuid| uuid.into())
            .collect::<Vec<bson::uuid::Uuid>>();

        // 拼接数据
        let user_uncheck = UserPropertyUncheck::builder()
            .mob_id(mob_id.mob_id)
            .datasource_push(datasource_uuids)
            .build();
        // 验证数据
        let user_checked: UserPropertyChecked =
            UserPropertyChecker::lite_check(user_uncheck).await?;
        // 将用户信息存入数据库
        mongo.ceobe().user().property().create(user_checked).await?;
        Ok(())
    }

    /// 获取用户数据源配置
    pub async fn get_datasource_by_user(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate, mob_id: UserMobId,
    ) -> LogicResult<DatasourceConfig> {
        // 获取所有数据源的uuid列表
        // 获取用户数据源配置
        let (datasource_list, user_datasource_config) = future::join(
            db.fetcher().datasource().find_all_uuid(),
            mongo
                .ceobe()
                .user()
                .property()
                .find_datasource_list_by_mob(mob_id.clone()),
        )
        .await;

        let user_datasource_config = user_datasource_config?;
        let datasource_set: HashSet<Uuid> =
            HashSet::from_iter(datasource_list?);
        let user_config_set: HashSet<bson::Uuid> =
            HashSet::from_iter(user_datasource_config.clone());

        // 获取用户设置有且数据源存在的列表
        let resp = DatasourceConfig {
            datasource_config: user_config_set
                .into_iter()
                .filter(|uuid| {
                    datasource_set.contains(&uuid.to_owned().into())
                })
                .map(|bson_uuid| bson_uuid.into())
                .collect::<Vec<uuid::Uuid>>(),
        };

        // 将删除过已不存在的数据源列表存回数据库
        // 异步执行，无论成功与否都继续~
        if resp.datasource_config.len() < user_datasource_config.len() {
            tokio::spawn(mongo.ceobe().user().property().update_datasource(
                mob_id.mob_id,
                vec_uuid_to_bson_uuid(resp.datasource_config.clone()),
            ));
            task::yield_now().await;
        }

        Ok(resp)
    }

    /// 更新用户数据源配置
    pub async fn update_datasource(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate,
        datasource_config: Vec<bson::Uuid>, mob_id: UserMobId,
    ) -> LogicResult<()> {
        let user_unchecked:UserPropertyUncheck = UserPropertyUncheck::builder().mob_id(mob_id.mob_id).datasource_push(datasource_config).build();
        let user_config:UserPropertyChecked = UserPropertyChecker::lite_check(user_unchecked).await?;

        // 判断是否所有数据源都存在
        db.fetcher().datasource().all_exist_by_uuid(vec_bson_uuid_to_uuid(user_config.datasource_push.clone())).await?.true_or_with(|| {
            warn!(user.datasources = ?user_config.datasource_push, user.datasources.exist = false);
            error::LogicError::DatasourceConfigOperateError(FetcherDatasourceOperateError::DatasourcesNotFound)
        })?;

        // 更新用户蹲饼器数据
        mongo
            .ceobe()
            .user()
            .property()
            .update_datasource(
                user_config.mob_id,
                user_config.datasource_push,
            )
            .await?;

        Ok(())
    }
}
