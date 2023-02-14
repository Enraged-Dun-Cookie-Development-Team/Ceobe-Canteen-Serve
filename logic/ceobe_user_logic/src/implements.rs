use std::ops::Deref;

use crate::error;
use crate::utils::vec_uuid_to_bson_uuid;
use crate::view::DatasourceConfig;
use crate::{error::LogicResult, view::MobIdReq};
use checker::LiteChecker;
use futures::{future, FutureExt};
use mongo_models::ceobe::user::check::user_checker::UserUncheck;
use mongo_models::ceobe::user::operate::OperateError;
use mongo_models::{
    ceobe::user::{
        check::user_checker::UserChecker, models::UserChecked,
        operate::ToUserOperate,
    },
    mongo_connection::MongoDatabaseOperate,
    mongodb::bson,
};
use sql_models::{
    fetcher::ToFetcherOperate,
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr},
        SqlDatabaseOperate,
    },
};
use tracing::{info_span, warn};
use tracing::instrument::Instrument;
use tokio::task;

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
        mongo.user().create(user_checked).await?;
        Ok(())
    }

    /// 获取用户数据源配置
    pub async fn get_datasource_by_user(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate, mob_id: MobIdReq,
    ) -> LogicResult<DatasourceConfig> {
        // TODO: 优化为中间件，放在用户相关接口判断用户是否存在
        // 判断用户是否存在
        let true = mongo.user().is_exist_user(
            &mob_id.mob_id,
        )
        .await? else {
            warn!(user.mob_id = %mob_id.mob_id, newUser.mob_id.exist = false);
            return Err(error::LogicError::CeobeUserOperateError(OperateError::UserMobIdNotExist(mob_id.mob_id)))
        };


        // 获取所有数据源的uuid列表
        // 获取用户数据源配置
        let (datasource_list, user_datasource_config) = future::join(
            db.fetcher_operate().datasource().find_all_uuid(),
            mongo.user().find_datasource_list_by_mob(mob_id.clone().into()),
        )
        .await;

        let datasource_list = datasource_list?;
        let user_datasource_config = user_datasource_config?;

        // 获取用户设置有且数据源存在的列表
        let mut resq = DatasourceConfig::new();
        resq.datasource_config = user_datasource_config
            .clone()    
            .into_iter()
            .filter(|uuid| datasource_list.contains(&uuid.to_owned().into()))
            .map(|bson_uuid| bson_uuid.into())
            .collect::<Vec<uuid::Uuid>>();

        // 将删除过已不存在的数据源列表存回数据库
        // 异步执行，无论成功与否都继续~
        if resq.datasource_config.len() < user_datasource_config.len() {
            tokio::spawn(
                mongo.user().update_datasource(mob_id.mob_id, vec_uuid_to_bson_uuid(resq.datasource_config.clone()))
            );
            task::yield_now().await;
        }

        Ok(resq)
    }
}
