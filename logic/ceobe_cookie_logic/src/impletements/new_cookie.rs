use ceobe_cookie::ToCeobe;
use ceobe_qiniu_upload::QiniuManager;
use ceobe_user::ToCeobeUser;
use db_ops_prelude::{get_connect::GetDatabaseConnect, SqlDatabaseOperate};
use fetcher::{
    datasource_combination::DatasourceCombinationOperate,
    datasource_config::DatasourceOperate,
};
use futures::future;
use mob_push_server::PushManager;
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use qiniu_service::QiniuService;
use qq_channel_warning::{LogRequest, LogType, QqChannelGrpcService};
use redis_connection::RedisConnect;

use crate::{
    error::{LogicError, LogicResult},
    impletements::CeobeCookieLogic,
    view::{NewCookieReq, PushInfo},
};

impl CeobeCookieLogic {
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate, sql: SqlDatabaseOperate,
        redis_client: RedisConnect, mut mob: PushManager,
        qq_channel: QqChannelGrpcService, qiniu: QiniuManager,
        new_cookies: Vec<NewCookieReq>,
    ) -> LogicResult<()> {
        // 处理数组为空的情况
        if new_cookies.is_empty() {
            return Ok(());
        }
        let db = sql.get_connect();
        // 查询数据源相关信息
        let datasource_info =
            DatasourceOperate::find_model_by_datasource_and_unique_key(
                db,
                &new_cookies[0].source.datasource,
                &new_cookies[0].source.unique,
            )
            .await?;
        let mut qq_channel_tmp = qq_channel.clone();
        // 最新的一个饼
        let newest_cookies = new_cookies.clone().into_iter().last().unwrap();
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
                        // 更新最新饼id对象储存
                        // 删除对象储存中的数据源组合文件
                        QiniuService::update_multi_datasource_comb(qiniu, Some(newest_cookies.cookie_id.to_string()), Some(newest_cookies.cookie_id.to_string()), qq_channel, redis_client, comb_ids, Some(format!("{}:{}", &datasource_info.datasource, &datasource_info.db_unique_key))).await;
                        Ok(())
                    },
                    Err(err) => Err(LogicError::from(err)),
                }
            }
        )
        .await;

        datasource_error?;
        qiniu_err?;

        Ok(())
    }
}
