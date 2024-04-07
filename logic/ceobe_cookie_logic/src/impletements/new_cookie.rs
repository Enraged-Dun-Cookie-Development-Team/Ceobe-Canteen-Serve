use ceobe_qiniu_upload::QiniuManager;
use futures::future;
use mob_push_server::PushManager;
use persistence::{
    ceobe_cookie::ToCeobe,
    ceobe_sync_cookie::SyncCookieOperate,
    ceobe_user::ToCeobeUser,
    fetcher::{
        datasource_combination::DatasourceCombinationOperate,
        datasource_config::DatasourceOperate,
    },
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
    operate::GetDatabaseConnect,
    redis::RedisConnect,
};
use qiniu_service::{model::DeleteObjectName, QiniuService};
use qq_channel_warning::{LogRequest, LogType, QqChannelGrpcService};

use crate::{
    error::{LogicError, LogicResult},
    impletements::CeobeCookieLogic,
    view::{NewCookieReq, PushInfo},
};

impl CeobeCookieLogic {
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate, sql: SqlDatabaseOperate, redis_client: RedisConnect, mut mob: PushManager,
        qq_channel: QqChannelGrpcService, qiniu: QiniuManager,
        new_cookies: Vec<NewCookieReq>,
    ) -> LogicResult<()> {
        // 处理数组为空的情况
        if new_cookies.is_empty() {
            return Ok(());
        }
        let db = sql.get_connect();
        let source = &new_cookies.first().unwrap().source;
        // 查询数据源相关信息
        let datasource_info =
            DatasourceOperate::find_model_by_datasource_and_unique_key(
                db,
                &source.datasource,
                &source.unique,
            )
            .await?;
        let mut qq_channel_tmp = qq_channel.clone();
        // 最新的一个饼
        let newest_cookie_id =
            new_cookies.last().map(|cookie| cookie.cookie_id);
        let (datasource_error, qiniu_err): (_,  Result<(), LogicError>) = future::join(
            async {
                // 查询用户列表
                let result = mongo
                    .ceobe()
                    .user()
                    .property()
                    .find_user_list_by_datasource(datasource_info.unique_id.into())
                    .await;
                match result {
                    Ok(user_list) => {
                        for new_cookie in new_cookies {
                            // mob推送新饼
                            let content = PushInfo::builder()
                            .content(new_cookie.content.text)
                            .datasource_name(datasource_info.nickname.clone())
                            .image_url(new_cookie.content.image_url)
                            .icon_url(datasource_info.avatar.clone())
                            .build();

                            if let Err(err) = mob.mob_push::<_, String, _>(&content, &user_list)
                                .await
                            {
                                qq_channel_tmp.send_logger(LogRequest::builder()
                                .level(LogType::Error)
                                .info("推送新饼失败".to_string())
                                .extra(format!("报错：{err}")).build()).await?;
                            }
                        }

                        Ok(())
                    },
                    Err(err) => Err(LogicError::from(err)),
                }
            },
            {
                let datasource_info = datasource_info.clone();
                async move {
                    // 获取新饼需要改变的数据源组合
                    let result =
                        DatasourceCombinationOperate::find_comb_id_by_one_datasource_raw(
                            db,
                            datasource_info.id,
                        )
                            .await;
                    match result {
                        Ok(comb_ids) => {
                            let mut operate = SyncCookieOperate::new(redis_client);
                            // 更新最新饼id对象储存
                            for comb_id in comb_ids.iter() {
                                operate.sync_cookie(
                                    newest_cookie_id,
                                    newest_cookie_id,
                                    comb_id.clone(),
                                    datasource_info.to_combin_id()).await?;
                            }
                            // 删除对象储存中的数据源组合文件
                            qiniu
                                .delete_many(
                                    comb_ids
                                        .into_iter()
                                        .map(DeleteObjectName::new)
                                        .collect())
                                .await
                                .map_err(LogicError::from)?;
                            Ok(())
                        }
                        Err(err) => Err(LogicError::from(err)),
                    }
                }
            }
        )
        .await;

        datasource_error?;
        qiniu_err?;

        Ok(())
    }
}
