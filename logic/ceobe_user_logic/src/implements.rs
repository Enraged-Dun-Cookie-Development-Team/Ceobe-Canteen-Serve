use std::collections::HashSet;

use abstract_database::{ceobe::ToCeobe, fetcher::ToFetcher};
use bitmap_convert::base70::BitmapBase70Conv;
use bitmaps::Bitmap;
use bnum::{types::U256, BUint};
use ceobe_qiniu_upload::QiniuUploader;
use ceobe_user::ToCeobeUser;
use checker::LiteChecker;
use db_ops_prelude::{
    bool_or::TrueOrError,
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::user_property::{
        check::user_checker::{UserPropertyChecker, UserPropertyUncheck},
        models::{UserMobId, UserPropertyChecked},
    },
    mongodb::bson,
    SqlDatabaseOperate, sql_models::fetcher::datasource_combination::models::model_datasource_combination::CombinationInfo,
};
use fetcher::{datasource_config::{
    OperateError as FetcherDatasourceOperateError, ToDatasource,
}, datasource_combination::ToDatasourceCombination};
use futures::future;
use tokio::task;
use tracing::warn;
use uuid::Uuid;
use uuids_convert::{vec_bson_uuid_to_uuid, vec_uuid_to_bson_uuid};

use crate::{
    error,
    error::{LogicResult, LogicError},
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
            .datasource_push(datasource_uuids.clone())
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
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate,
        mob_id: UserMobId, uploader: QiniuUploader,
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

        // 去除已被删除的数据源后的结果
        let handle_user_set = user_config_set
            .into_iter()
            .filter(|uuid| {
                datasource_set.contains(&uuid.to_owned().into())
            })
            .map(|bson_uuid| bson_uuid.into())
            .collect::<Vec<uuid::Uuid>>();

        // 生成组合id并返回给用户
        let comb_ids = Self::get_datasources_comb_ids(handle_user_set.clone(), db, uploader).await?;

        // 获取用户设置有且数据源存在的列表
        let resp = DatasourceConfig {
            datasource_config: handle_user_set,
            datasource_comb_id: comb_ids
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
        let user_unchecked: UserPropertyUncheck =
            UserPropertyUncheck::builder()
                .mob_id(mob_id.mob_id)
                .datasource_push(datasource_config)
                .build();
        let user_config: UserPropertyChecked =
            UserPropertyChecker::lite_check(user_unchecked).await?;

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
                user_config.datasource_push.clone(),
            )
            .await?;

        Ok(())
    }

    async fn get_datasources_comb_ids(datasource_uuid: Vec<Uuid>, db: SqlDatabaseOperate, uploader: QiniuUploader) -> LogicResult<String> {
        // 通过Vec<uuids>获取Vec<i32>
        let datasource_ids = db.fetcher().datasource().find_ids_by_uuids(datasource_uuid).await?;

        // 根据数据库id生成bitmap
        let mut comb_ids_map = Bitmap::<256>::new();
        datasource_ids.into_iter().for_each(|id| {comb_ids_map.set(id as usize, true); ()});

        let comb_id = comb_ids_map.to_base_70()?;

        // 转换bitmap成u64数组
        let value = U256::from_radix_le(comb_ids_map.as_bytes(), 256).ok_or(bitmap_convert::error::Error::from(bitmap_convert::error::Error::LargeThen256))?;
        let datasource_vec: [u64; 4] = value.into();
        
        // TODO: 上传数据源组合到对象储存

        // 创建数据源组合并记录
        let info = CombinationInfo {
            combination_id: comb_id.clone(),
            bitmap1: datasource_vec[0],
            bitmap2: datasource_vec[1],
            bitmap3: datasource_vec[2],
            bitmap4: datasource_vec[3],
        };
        db.fetcher().datasource_combination().create(info).await?;


        // 转成特定格式字符串
        Ok(comb_id)
    }
}
