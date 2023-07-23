use bool_or::TrueOrError;
use ceobe_qiniu_upload::QiniuManager;
use persistence::fetcher::{
    config::ConfigOperate,
    datasource_combination::DatasourceCombinationOperate,
    datasource_config::DatasourceOperate, platform_config::PlatformOperate,
    models::datasource_config::{
        checkers::FetcherDatasourceConfig,
        models::model_datasource_config::DatasourcePlatform,
    },
};
use qiniu_service::QiniuService;
use qq_channel_warning::QqChannelGrpcService;
use persistence::redis::RedisConnect;
use scheduler_notifier::SchedulerNotifier;
use persistence::mysql::SqlDatabaseOperate;
use persistence::operate::{GetDatabaseConnect,GetDatabaseTransaction,TransactionOps};

use crate::{
    error::{LogicError, LogicResult},
    implements::FetcherConfigLogic,
};

impl FetcherConfigLogic {
    /// 新建数据源配置
    pub async fn create_datasource_config(
        db: SqlDatabaseOperate, datasource_config: FetcherDatasourceConfig,
    ) -> LogicResult<()> {
        let db = db.get_connect();
        // 验证平台存在
        PlatformOperate::exist_by_type_id(db, &datasource_config.platform)
            .await?
            .true_or_with(|| LogicError::PlatformNotFound)?;
        // 创建数据源
        DatasourceOperate::create(db, datasource_config).await?;
        Ok(())
    }

    /// 删除一个数据源
    pub async fn delete_datasource_by_id(
        notifier: &SchedulerNotifier, db: SqlDatabaseOperate,
        mut qq_channel: QqChannelGrpcService, qiniu: QiniuManager,
        mut redis_client: RedisConnect, id: i32,
    ) -> LogicResult<()> {
        // 开事务
        let ctx = db.get_transaction().await?;

        // 删除蹲饼器配置中的所有有datasource_id的配置
        ConfigOperate::delete_by_datasource_id(&ctx, id).await?;

        let DatasourcePlatform { platform } =
            DatasourceOperate::find_platform_by_id(&ctx, id).await?;

        // 删除数据源
        DatasourceOperate::delete_one(&ctx, id).await?;

        // 删除数据源组合
        let comb_ids =
            DatasourceCombinationOperate::find_comb_id_by_one_datasource_raw(
                &ctx, id,
            )
            .await?;
        let mut delete_comb_ids = Vec::<String>::new();
        // 删除对象储存中的数据源组合文件
        for comb_id in comb_ids {
            if QiniuService::delete_datasource_comb(
                &qiniu,
                &mut qq_channel,
                &mut redis_client,
                comb_id.clone(),
            )
            .await
            .is_ok()
            {
                delete_comb_ids.push(comb_id);
            }
        }

        // 删除数据源组合
        DatasourceCombinationOperate::delete_by_datasource(
            &ctx,
            delete_comb_ids,
        )
        .await?;

        // 提交事务
        ctx.submit().await?;
        notifier.notify_platform_update(platform).await;

        Ok(())
    }
}
