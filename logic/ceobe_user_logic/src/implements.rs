use std::collections::HashSet;

use ceobe_user::{user::OperateError as CeobeUserOperateError, ToCeobeUser};
use checker::LiteChecker;
use db_ops_prelude::{
    bool_or::TrueOrError,
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::user::{
        check::user_checker::{UserChecker, UserUncheck},
        models::UserChecked,
    },
    mongodb::bson,
    sql_models::fetcher::{
        datasource_config::operate::OperateError as FetcherDatasourceOperateError,
        ToFetcherOperate,
    },
    SqlDatabaseOperate,
};
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
            .fetcher_operate()
            .datasource()
            .find_all_uuid()
            .await?
            .into_iter()
            .map(|uuid| uuid.into())
            .collect::<Vec<bson::uuid::Uuid>>();

        // 拼接数据
        let user_uncheck = UserUncheck::builder()
            .mob_id(mob_id.mob_id)
            .datasource_push(datasource_uuids)
            .build();
        // 验证数据
        let user_checked: UserChecked =
            UserChecker::lite_check(user_uncheck).await?;
        // 将用户信息存入数据库
        mongo.ceobe_user().user().create(user_checked).await?;
        Ok(())
    }

    /// 获取用户数据源配置
    pub async fn get_datasource_by_user(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate, mob_id: MobIdReq,
    ) -> LogicResult<DatasourceConfig> {
        // TODO: 优化为中间件，放在用户相关接口判断用户是否存在
        // 判断用户是否存在
        mongo.ceobe_user().user().is_exist_user(
            &mob_id.mob_id,
        )
        .await?.true_or_with(|| {
            warn!(user.mob_id = %mob_id.mob_id, newUser.mob_id.exist = false);
            error::LogicError::CeobeUserOperateError(CeobeUserOperateError::UserMobIdNotExist(mob_id.mob_id.clone()))
        })?;

        // 获取所有数据源的uuid列表
        // 获取用户数据源配置
        let (datasource_list, user_datasource_config) = future::join(
            db.fetcher_operate().datasource().find_all_uuid(),
            mongo
                .ceobe_user()
                .user()
                .find_datasource_list_by_mob(mob_id.clone().into()),
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
            tokio::spawn(mongo.ceobe_user().user().update_datasource(
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
        user_config: UserChecked,
    ) -> LogicResult<()> {
        // TODO: 优化为中间件，放在用户相关接口判断用户是否存在
        // 判断用户是否存在
        mongo.ceobe_user().user().is_exist_user(
            &user_config.mob_id,
        )
        .await?.true_or_with(|| {
            warn!(user.mob_id = %user_config.mob_id, newUser.mob_id.exist = false);
            error::LogicError::CeobeUserOperateError(CeobeUserOperateError::UserMobIdNotExist(user_config.mob_id.clone()))
        })?;

        // 判断是否所有数据源都存在
        db.fetcher_operate().datasource().all_exist_by_uuid(vec_bson_uuid_to_uuid(user_config.datasource_push.clone())).await?.true_or_with(|| {
            warn!(user.datasources = ?user_config.datasource_push, user.datasources.exist = false);
            error::LogicError::DatasourceConfigOperateError(FetcherDatasourceOperateError::DatasourcesNotFound)
        })?;

        // 更新用户蹲饼器数据
        mongo
            .ceobe_user()
            .user()
            .update_datasource(
                user_config.mob_id,
                user_config.datasource_push,
            )
            .await?;

        Ok(())
    }
}
